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

use std::borrow::Borrow as _;
use std::error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::task::Poll;

use auto_traits::pointer_impls;

use crate::actions::ConstructableAction;
use crate::auxillary::{Actored, Authored, Identifiable};
use crate::collections::Recipient;
use crate::collections::set::{Set, SetAsync, SetSync};
use crate::messages::ConstructableMessage;



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
pub enum Error<I, E> {
    /// The agent attempted to enact an action of someone else.
    IllegalEnact { agent: I, author: I },
    /// The agent attempted to gossip a message they did not know.
    IllegalGossip { agent: I },
    /// The agent attempted to state a message of someone else.
    IllegalState { agent: I, author: I },
    /// One of the set's interactions failed.
    Set(E),
}
impl<I: Debug, E: Display> Display for Error<I, E> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::IllegalEnact { agent, author } => write!(f, "Agent {agent:?} is not allowed to enact a message by {author:?}"),
            Self::IllegalGossip { agent } => write!(f, "Agent {agent:?} is not allowed to gossip a message they did not know"),
            Self::IllegalState { agent, author } => write!(f, "Agent {agent:?} is not allowed to state a message by {author:?}"),
            Self::Set(err) => err.fmt(f),
        }
    }
}
impl<I: Debug, E: error::Error> error::Error for Error<I, E> {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IllegalEnact { .. } => None,
            Self::IllegalGossip { .. } => None,
            Self::IllegalState { .. } => None,
            Self::Set(err) => err.source(),
        }
    }
}





/***** AUXILLARY *****/
/// Defines the view that agents or synchronizers have on the runtime.
#[derive(Clone, Copy, Debug)]
pub struct View<I: ?Sized + ToOwned, A, S, E> {
    /// The identifier of the agent for who this view is.
    pub id:      I::Owned,
    /// The set of agreements that have been formulated.
    pub agreed:  A,
    /// The set of messages that have been stated (and visible to this agent!).
    pub stated:  S,
    /// The set of actions that have been enacted (and visible to this agent!).
    pub enacted: E,
}

impl<I: ?Sized + ToOwned, A, S, E> View<I, A, S, E> {
    /// Have the agent state a message to their own view.
    ///
    /// # Arguments
    /// - `msg`: The message to state.
    ///
    /// # Errors
    /// This function errors if adding the message to the internal `S`tatements-set fails to add
    /// the new message, or if the agent attempted to publish a message not theirs.
    #[inline]
    pub fn state<MS>(&mut self, msg: MS) -> Result<(), Error<I::Owned, S::Error>>
    where
        I::Owned: Clone + PartialEq,
        S: SetAsync<I, MS>,
        MS: Authored<AuthorId = I>,
    {
        // Check this agent is publishing to their own view
        let author_id: I::Owned = msg.author_id().to_owned();
        if self.id != author_id {
            return Err(Error::IllegalState { agent: self.id.clone(), author: author_id });
        }

        // Now publish the message
        self.stated.add(Recipient::One(&author_id.borrow()), msg).map_err(Error::Set)
    }

    /// Have the agent enact an action to their own view.
    ///
    /// # Arguments
    /// - `act`: The action to enact.
    ///
    /// # Errors
    /// This function errors if adding the message to the internal `S`tatements-set fails to add
    /// the new message.
    #[inline]
    pub fn enact<AS>(&mut self, act: AS) -> Result<(), Error<I::Owned, E::Error>>
    where
        I::Owned: Clone + PartialEq,
        E: SetAsync<AS::ActorId, AS>,
        AS: Actored<ActorId = I>,
    {
        // Check this agent is publishing to their own view
        let author_id: I::Owned = act.actor_id().to_owned();
        if self.id != author_id {
            return Err(Error::IllegalEnact { agent: self.id.clone(), author: author_id });
        }

        // Now publish the message
        self.enacted.add(Recipient::One(&author_id.borrow()), act).map_err(Error::Set)
    }

    /// Agree on a new agreement.
    ///
    /// Specifically, replaces all of the agreements with the given list.
    ///
    /// # Arguments
    /// - `agrees`: An iterator yielding agreements to put in the list.
    ///
    /// # Errors
    /// This function errors if we failed to clear the existing list or if we added any of the
    /// agreements yielded by `agrees`.
    #[inline]
    pub fn agree<MS>(&mut self, agrees: impl IntoIterator<Item = MS>) -> Result<(), Error<I::Owned, A::Error>>
    where
        A: SetSync<MS>,
    {
        // Reset, then write the new agreements
        self.agreed.clear().map_err(Error::Set)?;
        for agree in agrees {
            self.agreed.add(agree).map_err(Error::Set)?;
        }
        Ok(())
    }



    /// Gossips a particular message to a new recipient.
    ///
    /// Note, though, that the message must already be in the agent's view for this to be allowed.
    ///
    /// # Arguments
    /// - `to`: Some [`Recipient`] to gossip the message to.
    /// - `message`: The message to gossip.
    ///
    /// # Errors
    /// This function errors if we failed to access the list of stated messages or if the current
    /// agent did not know the `message`.
    #[inline]
    pub fn gossip<MS>(&mut self, to: Recipient<&I>, message: MS) -> Result<(), Error<I::Owned, S::Error>>
    where
        I::Owned: Clone,
        S: SetAsync<I, MS>,
        MS: Authored<AuthorId = I> + PartialEq,
    {
        // Check if this agent knew the message
        let mut found = false;
        for msg in self.stated.iter().map_err(Error::Set)? {
            if msg == &message {
                // They did
                found = true;
                break;
            }
        }
        if !found {
            return Err(Error::IllegalGossip { agent: self.id.clone() });
        }

        // If they did, send it to the recipients
        self.stated.add(to, message).map_err(Error::Set)
    }
}





/***** LIBRARY *****/
/// Defines how any runtime interfaces with agents.
///
/// Agents are the main actors in the JustAct framework. They use information in all the sets
/// (synchronized and otherwise) to publish new content in asynchronized sets they have access to.
///
/// # Generics
/// - `MP`: The type of the message payloads supported by this Synchronizer.
#[pointer_impls(T = U)]
pub trait Agent<MP>: Identifiable
where
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
    fn poll<A, S, E, SM, SA>(&mut self, view: View<Self::Id, A, S, E>) -> Result<Poll<()>, Self::Error>
    where
        A: Set<SM>,
        S: SetAsync<Self::Id, SM>,
        E: SetAsync<Self::Id, SA>,
        SM: ConstructableMessage<AuthorId = Self::Id, Payload = MP>,
        SA: ConstructableAction<ActorId = Self::Id, Message = SM>;
}



/// Defines how any runtime interfaces with synchronizers.
///
/// Synchronizers are a special kind of actors that have the power to update synchronized sets.
/// Like agents, they may use information available in any kind of set to do so.
///
/// # Generics
/// - `MP`: The type of the message payloads supported by this Synchronizer.
#[pointer_impls(T = U)]
pub trait Synchronizer<MP>: Identifiable
where
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
    fn poll<A, S, E, SM, SA>(&mut self, view: View<Self::Id, A, S, E>) -> Result<Poll<()>, Self::Error>
    where
        A: SetSync<SM>,
        S: SetAsync<Self::Id, SM>,
        E: SetAsync<Self::Id, SA>,
        SM: ConstructableMessage<AuthorId = Self::Id, Payload = MP>,
        SA: ConstructableAction<ActorId = Self::Id, Message = SM>;
}
