//  RUNTIME.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 17:11:17
//  Last edited:
//    29 Jan 2025, 22:03:37
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the toplevel interface of a [`Runtime`]. While this is not
//!   in the ontology, it does hint at how it is supposed to be used.
//

use std::error;

use auto_traits::pointer_impls;

use crate::actors::{Agent, Synchronizer};


/***** LIBRARY *****/
/// Defines the toplevel [`Runtime`], which brings the ontology together.
#[pointer_impls]
pub trait Runtime {
    /// Defines the type of identifiers for agents.
    type AgentId: ?Sized + ToOwned;
    /// Defines the type of identifiers for synchronizers.
    type SynchronizerId: ?Sized + ToOwned;
    /// Defines the type of payloads used in the runtime.
    type Payload: ?Sized + ToOwned;

    /// Any errors thrown by the runtime.
    type Error: 'static + Send + error::Error;


    /// Runs this runtime for a given set of agents.
    ///
    /// # Arguments
    /// - `agents`: Something yielding the total set of [`Agent`]s to run.
    /// - `synchronizer`: The [`Synchronizer`] that can influence synchronized sets and/or when
    ///   the runtime stops.
    ///
    /// # Errors
    /// This function errors whenever anything in the set goes wrong.
    fn run<A>(
        &mut self,
        agents: impl IntoIterator<Item = A>,
        synchronizer: impl Synchronizer<Self::Payload, Id = Self::SynchronizerId>,
    ) -> Result<(), Self::Error>
    where
        A: Agent<Self::Payload, Id = Self::AgentId>;
}
