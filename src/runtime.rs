//  RUNTIME.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 17:11:17
//  Last edited:
//    11 Dec 2024, 15:38:18
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the toplevel interface of a [`Runtime`]. While this is not
//!   in the ontology, it does hint at how it is supposed to be used.
//

use std::error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::hash::Hash;

use crate::actions::Action;
use crate::actors::{Agent, Synchronizer};
use crate::agreements::Agreement;
use crate::messages::{Message, MessageSet};
use crate::policies::{Extractable, Policy};
use crate::sets::{InfallibleSet, Set, SetMut};
use crate::times::Times;


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
impl<T, A, S, E, MI, MA, MC, MT> View<T, A, S, E>
where
    A: Set<Elem = Agreement<MI, MA, MC, MT>>,
    S: Set<Elem = Message<MI, MA, MC>>,
    E: Set<Elem = Action<MI, MA, MC, MT>>,
    MI: Eq + Hash,
{
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
    pub fn get_statement<'s>(&'s self, id: &MI) -> Result<Option<&'s Message<MI, MA, MC>>, Error<MI, A::Error, S::Error, E::Error>>
    where
        MI: Clone,
        MT: 's,
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
            // Try the basis first
            if let Some(msg) = <Agreement<MI, MA, MC, MT> as InfallibleSet>::get(&act.basis, id) {
                return Ok(Some(msg));
            }

            // Then the justification
            if let Some(msg) = <MessageSet<MI, MA, MC> as InfallibleSet>::get(&act.justification, id) {
                return Ok(Some(msg));
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
    pub fn statements<'s>(&'s self) -> Result<impl Iterator<Item = &'s Message<MI, MA, MC>>, Error<MI, A::Error, S::Error, E::Error>>
    where
        MI: 's,
        MA: 's,
        MC: 's,
    {
        let aiter = self.agreed.iter().map_err(|err| Error::StatementsIter { err: OneOfSetError::Agreements(err) })?.map(|a| &a.message);
        let siter = self.stated.iter().map_err(|err| Error::StatementsIter { err: OneOfSetError::Statements(err) })?;
        let eiter = self
            .enacted
            .iter()
            .map_err(|err| Error::StatementsIter { err: OneOfSetError::Enactments(err) })?
            .flat_map(|e| <MessageSet<MI, MA, MC> as InfallibleSet>::iter(&e.justification));
        Ok(aiter.chain(siter).chain(eiter))
    }
}





/***** LIBRARY *****/
/// Defines the toplevel [`Runtime`], which brings the ontology together.
pub trait Runtime {
    /// Defines the type of message- and action identifiers.
    type Id: Eq + Hash;
    /// Defines the type of agent identifiers.
    type AgentId: Eq + Hash;
    /// Defines the contents of messages.
    type Contents: Extractable;
    /// Defines the representation of a timestamp.
    type Timestamp: Eq + Ord;
    /// Defines the policy extracted from messages.
    type Policy: Policy;
    /// Defines the set of synchronized times.
    type Times: SetMut + Times<Timestamp = Self::Timestamp>;
    /// Defines the set of synchronized agreements.
    type Agreements: SetMut<Elem = Agreement<Self::Id, Self::AgentId, Self::Contents, Self::Timestamp>>;
    /// Defines the set of statements.
    type Statements: SetMut<Elem = Message<Self::Id, Self::AgentId, Self::Contents>>;
    /// Defines the set of enacted actions.
    type Enactments: SetMut<Elem = Action<Self::Id, Self::AgentId, Self::Contents, Self::Timestamp>>;
    /// Any errors thrown by the runtime.
    type Error: error::Error;


    /// Runs this runtime for a given set of agents.
    ///
    /// # Arguments
    /// - `agents`: Something yielding the total set of [`Agent`]s to run.
    /// - `synchronizer`: The [`Synchronizer`] that can influence synchronized sets and/or when
    ///   the runtime stops.
    ///
    /// # Errors
    /// This function errors whenever anything in the set goes wrong.
    fn run<A>(&mut self, agents: impl IntoIterator<Item = A>, synchronizer: impl Synchronizer) -> Result<(), Self::Error>
    where
        A: Agent;
}
