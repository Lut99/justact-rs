//  RUNTIME.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 17:11:17
//  Last edited:
//    14 Jan 2025, 17:07:04
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
    /// Defines the type of identifiers for messages.
    type MessageId: ?Sized;
    /// Defines the type of identifiers for actions.
    type ActionId: ?Sized;
    /// Defines the type of identifiers for agents.
    type AgentId: ?Sized;
    /// Defines the type of identifiers for synchronizers.
    type SynchronizerId: ?Sized;
    /// Defines the type of timestamp used for this impl.
    type Timestamp;

    /// Any errors thrown by the runtime.
    type Error: 'static + error::Error;


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
        synchronizer: impl Synchronizer<Self::MessageId, Self::ActionId, Self::Timestamp, Id = Self::SynchronizerId>,
    ) -> Result<(), Self::Error>
    where
        A: Agent<Self::MessageId, Self::ActionId, Self::Timestamp, Id = Self::AgentId>;
}
