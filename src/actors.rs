//  ACTORS.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 11:00:07
//  Last edited:
//    29 Jan 2025, 22:03:45
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the two types of actors in the ontology: [_agents_](Agent),
//!   who read synchronized- and asynchronized sets and write to
//!   asynchronized sets; and [_synchronizers_](Synchronizer), who read
//!   synchronized- and asynchronized sets and write to synchronized sets.
//

use std::error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::hash::Hash;
use std::task::Poll;

use auto_traits::pointer_impls;

use crate::actions::{Action, ConstructableAction};
use crate::agreements::Agreement;
use crate::auxillary::{Authored, Identifiable};
use crate::collections::map::{InfallibleMap, Map, MapAsync, MapSync};
use crate::messages::{ConstructableMessage, MessageSet};
use crate::times::{Times, TimesSync};



/***** ERRORS *****/
/// Defines an error that is one of the given ones.
#[derive(Debug)]
pub enum OneOfSetError<EA, ES, EE> {
    /// The agreement set failed.
    Agreements(EA),
    /// The statements set failed.
    Statements(ES),
    /// The enacted set failed.
    Enactments(EE),
}
impl<EA: Display, ES: Display, EE: Display> Display for OneOfSetError<EA, ES, EE> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::Agreements(a) => a.fmt(f),
            Self::Statements(s) => s.fmt(f),
            Self::Enactments(e) => e.fmt(f),
        }
    }
}
impl<EA: error::Error, ES: error::Error, EE: error::Error> error::Error for OneOfSetError<EA, ES, EE> {
    #[inline]
    fn source(&self) -> Option<&(dyn 'static + error::Error)> {
        match self {
            Self::Agreements(a) => a.source(),
            Self::Statements(s) => s.source(),
            Self::Enactments(e) => e.source(),
        }
    }
}

/// Defines errors that originate from the [`View`].
#[derive(Debug)]
pub enum Error<I, EA, ES, EE> {
    /// Failed to get a particular statement.
    StatementGet { id: I, err: OneOfSetError<EA, ES, EE> },
    /// Failed to create an iterator over all statements.
    StatementsIter { err: OneOfSetError<EA, ES, EE> },
}
impl<I: Debug, EA, ES, EE> Display for Error<I, EA, ES, EE> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::StatementGet { id, .. } => write!(f, "Failed to get statement {id:?}"),
            Self::StatementsIter { .. } => write!(f, "Failed to iterate over all the statements in a view"),
        }
    }
}
impl<I: Debug, EA: 'static + error::Error, ES: 'static + error::Error, EE: 'static + error::Error> error::Error for Error<I, EA, ES, EE> {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::StatementGet { err, .. } => Some(err),
            Self::StatementsIter { err } => Some(err),
        }
    }
}





/***** AUXILLARY *****/
/// Defines the view that agents or synchronizers have on the runtime.
#[derive(Clone, Copy, Debug)]
pub struct View<T, A, S, E> {
    /// The set of times that can possibly exist, including one current one.
    pub times:   T,
    /// The set of agreements that have been formulated.
    pub agreed:  A,
    /// The set of messages that have been stated (and visible to this agent!).
    pub stated:  S,
    /// The set of actions that have been enacted (and visible to this agent!).
    pub enacted: E,
}
impl<T, A, S, E> View<T, A, S, E> {
    /// Returns a message with a particular ID across all statements in this view.
    ///
    /// This searches not just the statements in [`View::stated`], but also those embedded in
    /// [`View::agreed`] and [`View::enacted`].
    ///
    /// # Arguments
    /// - `id`: The identifier of the message to search for.
    ///
    /// # Returns
    /// A reference to the matching [`Message`], or else [`None`].
    ///
    /// # Errors
    /// This function can error if any of the agreements or statements sets errors when an element
    /// is being retrieved. In addition, the enacted set may throw an error if iterating over it
    /// failed.
    pub fn get_statement<'s, SM, SA>(&'s self, id: &SM::Id) -> Result<Option<&'s SM>, Error<SM::Id, A::Error, S::Error, E::Error>>
    where
        T: Times,
        A: Map<Agreement<SM, T::Timestamp>>,
        S: Map<SM>,
        E: Map<SA>,
        SM: Authored + Identifiable,
        SM::Id: Clone,
        SA: 's + Action<Message = SM, Timestamp = T::Timestamp>,
        SA::Id: ToOwned,
        SA::ActorId: ToOwned,
    {
        match self.agreed.get(id) {
            Ok(Some(agree)) => return Ok(Some(&agree.message)),
            Ok(None) => {},
            Err(err) => return Err(Error::StatementGet { id: id.clone(), err: OneOfSetError::Agreements(err) }),
        }
        match self.stated.get(id) {
            Ok(Some(msg)) => return Ok(Some(msg)),
            Ok(None) => {},
            Err(err) => return Err(Error::StatementGet { id: id.clone(), err: OneOfSetError::Statements(err) }),
        }
        for act in self.enacted.iter().map_err(|err| Error::StatementGet { id: id.clone(), err: OneOfSetError::Enactments(err) })? {
            // First search the basis.
            // NOTE: We don't search the payload due to lifetime issues, and it's basis + extras
            // anyway. Further, we assume that we can't predict the ID of the actor anyway.
            let agr = act.basis();
            if agr.message.id() == id {
                return Ok(Some(&agr.message));
            }

            // Then search the extras
            for extra in <MessageSet<_> as InfallibleMap<_>>::iter(act.extra()) {
                if extra.id() == id {
                    return Ok(Some(extra));
                }
            }
        }
        Ok(None)
    }

    /// Returns an iterator over all the statements in this view.
    ///
    /// This is not just the statements in [`View::stated`], but also those embedded in
    /// [`View::agreed`] and [`View::enacted`].
    ///
    /// # Returns
    /// An [`Iterator`] yielding references to [`Message`]s that represent all the statements in
    /// this view.
    ///
    /// # Errors
    /// This function can error if any of the nested sets errors when their iterator is being constructed.
    pub fn statements<'s, SM, SA>(&'s self) -> Result<impl 's + Iterator<Item = SM>, Error<<SM::Id as ToOwned>::Owned, A::Error, S::Error, E::Error>>
    where
        T: Times,
        A: Map<Agreement<SM, T::Timestamp>>,
        S: Map<SM>,
        E: Map<SA>,
        SM: 's + Clone + Identifiable,
        SM::Id: ToOwned,
        <SM::Id as ToOwned>::Owned: Eq + Hash,
        SA: 's + Action<Message = SM, Timestamp = T::Timestamp>,
        SA::Id: ToOwned,
        SA::ActorId: ToOwned,
    {
        let aiter = self.agreed.iter().map_err(|err| Error::StatementsIter { err: OneOfSetError::Agreements(err) })?.map(|a| a.message.clone());
        let siter = self.stated.iter().map_err(|err| Error::StatementsIter { err: OneOfSetError::Statements(err) })?.cloned();
        let eiter =
            self.enacted.iter().map_err(|err| Error::StatementsIter { err: OneOfSetError::Enactments(err) })?.flat_map(|e| e.payload().into_iter());
        Ok(aiter.chain(siter).chain(eiter))
    }
}




/***** LIBRARY *****/
/// Defines how any runtime interfaces with agents.
///
/// Agents are the main actors in the JustAct framework. They use information in all the sets
/// (synchronized and otherwise) to publish new content in asynchronized sets they have access to.
///
/// # Generics
/// - `MI`: The type of message IDs supported by this agent.
/// - `AI`: The type of action IDs supported by this agent.
/// - `MP`: The type of the message payloads supported by this Synchronizer.
/// - `TS`: The type of timestamp supported by this agent.
#[pointer_impls(T = U)]
pub trait Agent<MI, AI, MP, TS>: Identifiable
where
    MI: ?Sized + ToOwned,
    AI: ?Sized + ToOwned,
    MP: ?Sized + ToOwned,
    Self::Id: ToOwned,
{
    /// Any errors that this agent can throw during its execution.
    type Error: 'static + Send + error::Error;


    /// Polls this agent.
    ///
    /// This allows an agent to think. In particular, it will be polled by a
    /// [`Runtime`](crate::runtime::Runtime)'s busy loop (one way or another), and it is the
    /// opportunity for the agent to respond to changes in the system by doing changes to the
    /// system. I'm sure this is all very digital-twin approved.
    ///
    /// # Generics
    /// - `T`: The globally synchronized set of timestamps, including which one is current.
    /// - `A`: The globally synchronized set of agreements.
    /// - `S`: The local view on stated messages.
    /// - `E`: The local view on enacted actions.
    /// - `SM`: The type of messages carried within the sets above.
    /// - `SA`: The type of actions carried within the sets above.
    ///
    /// # Arguments
    /// - `view`: A runtime [`View`] that represents this agent's view on the current system.
    ///   How partial this view is, and how much is hidden behind the scenes, depends on the
    ///   simulation.
    ///
    /// # Returns
    /// A [`Poll`] which, can either:
    /// - be [`Poll::Ready`], indicating the agent has no more work to do (and can be deleted); or
    /// - a [`Poll::Pending`], indicating the agent wants to stick around.
    fn poll<T, A, S, E, SM, SA>(&mut self, view: View<T, A, S, E>) -> Result<Poll<()>, Self::Error>
    where
        T: Times<Timestamp = TS>,
        A: Map<Agreement<SM, TS>>,
        S: MapAsync<Self::Id, SM>,
        E: MapAsync<Self::Id, SA>,
        SM: ConstructableMessage<Id = MI, AuthorId = Self::Id, Payload = MP>,
        SA: ConstructableAction<Id = AI, ActorId = Self::Id, Message = SM, Timestamp = TS>;
}



/// Defines how any runtime interfaces with synchronizers.
///
/// Synchronizers are a special kind of actors that have the power to update synchronized sets.
/// Like agents, they may use information available in any kind of set to do so.
///
/// # Generics
/// - `MI`: The type of message IDs supported by this Synchronizer.
/// - `AI`: The type of action IDs supported by this Synchronizer.
/// - `MP`: The type of the message payloads supported by this Synchronizer.
/// - `TS`: The type of timestamp supported by this Synchronizer.
#[pointer_impls(T = U)]
pub trait Synchronizer<MI, AI, MP, TS>: Identifiable
where
    MI: ?Sized + ToOwned,
    AI: ?Sized + ToOwned,
    MP: ?Sized + ToOwned,
    Self::Id: ToOwned,
{
    /// Any errors that this synchronizer can throw during its execution.
    type Error: 'static + Send + error::Error;


    /// Polls this synchronizer.
    ///
    /// This allows the synchronizer to influence the decisions with "offline" synchronized
    /// information. In particular, it will be polled by a [`Runtime`](crate::runtime::Runtime)'s
    /// busy loop (one way or another), and it is the opportunity for the synchronizer to change
    /// synchronized sets based on the system's state.
    ///
    /// # Generics
    /// - `T`: The globally synchronized set of timestamps, including which one is current.
    /// - `A`: The globally synchronized set of agreements.
    /// - `S`: The local view on stated messages.
    /// - `E`: The local view on enacted actions.
    /// - `SM`: The type of messages carried within the sets above.
    /// - `SA`: The type of actions carried within the sets above.
    ///
    /// # Arguments
    /// - `view`: A runtime [`View`] that represents this synchronizer's view on the current
    ///   system. How partial this view is, and how much is hidden behind the scenes, depends on
    ///   the simulation.
    ///
    /// # Returns
    /// A [`Poll`] which, can either:
    /// - be [`Poll::Ready`], indicating the synchronizer has no more work to do (and can be
    ///   deleted); or
    /// - a [`Poll::Pending`], indicating the synchronizer wants to stick around.
    fn poll<T, A, S, E, SM, SA>(&mut self, view: View<T, A, S, E>) -> Result<Poll<()>, Self::Error>
    where
        T: TimesSync<Timestamp = TS>,
        A: MapSync<Agreement<SM, TS>>,
        S: MapAsync<Self::Id, SM>,
        E: MapAsync<Self::Id, SA>,
        SM: ConstructableMessage<Id = MI, AuthorId = Self::Id, Payload = MP>,
        SA: ConstructableAction<Id = AI, ActorId = Self::Id, Message = SM, Timestamp = TS>;
}
