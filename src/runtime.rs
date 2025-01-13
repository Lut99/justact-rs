//  RUNTIME.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 17:11:17
//  Last edited:
//    13 Jan 2025, 17:19:38
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the toplevel interface of a [`Runtime`]. While this is not
//!   in the ontology, it does hint at how it is supposed to be used.
//

use std::error;

use auto_traits::pointer_impls;

use crate::actions::Action;
use crate::actors::{Agent, Synchronizer};
use crate::agreements::Agreement;
use crate::collections::map::{MapAsync, MapSync};
use crate::collections::set::SetSync;
use crate::messages::Message;
use crate::times::Times;


/***** LIBRARY *****/
/// Defines the toplevel [`Runtime`], which brings the ontology together.
#[pointer_impls]
pub trait Runtime {
    /// Defines the type of identifiers for agents / synchronizers.
    type AgentId: ?Sized;

    /// Defines the type of messages in the runtime.
    type Message: Message;
    /// Defines the type of actions in the runtime.
    type Action: Action<Message = Self::Message, Timestamp = <Self::Times as Times>::Timestamp>;

    /// Defines the set of synchronized times.
    type Times: SetSync<<Self::Times as Times>::Timestamp> + Times;
    /// Defines the set of synchronized agreements.
    type Agreements: MapSync<Agreement<Self::Message, <Self::Times as Times>::Timestamp>>;
    /// Defines the set of statements.
    type Statements: MapAsync<Self::AgentId, Self::Message>;
    /// Defines the set of enacted actions.
    type Enactments: MapAsync<Self::AgentId, Self::Action>;

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
    fn run<A>(&mut self, agents: impl IntoIterator<Item = A>, synchronizer: impl Synchronizer<Id = Self::AgentId>) -> Result<(), Self::Error>
    where
        A: Agent<Id = Self::AgentId>;
}
