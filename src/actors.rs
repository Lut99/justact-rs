//  ACTORS.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 11:00:07
//  Last edited:
//    16 Dec 2024, 15:27:23
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the two types of actors in the ontology: [_agents_](Agent),
//!   who read synchronized- and asynchronized sets and write to
//!   asynchronized sets; and [_synchronizers_](Synchronizer), who read
//!   synchronized- and asynchronized sets and write to synchronized sets.
//

use std::error::Error;
use std::ops::ControlFlow;
use std::task::Poll;

use auto_traits::pointer_impls;

use crate::actions::Action;
use crate::agreements::Agreement;
use crate::auxillary::Identifiable;
use crate::messages::Message;
use crate::runtime::View;
use crate::sets::{Set, SetMut};
use crate::times::{Times, Timestamp};


/***** LIBRARY *****/
/// Defines how any runtime interfaces with agents.
///
/// Agents are the main actors in the JustAct framework. They use information in all the sets
/// (synchronized and otherwise) to publish new content in asynchronized sets they have access to.
#[pointer_impls(T = U)]
pub trait Agent: Identifiable {
    /// Any errors that this agent can throw during its execution.
    type Error: Error;


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
        T: Times,
        A: Set<Agreement<SM, T::Timestamp>>,
        S: SetMut<SM>,
        E: SetMut<SA>,
        SM: Message,
        SA: Action;
}



/// Defines how any runtime interfaces with synchronizers.
///
/// Synchronizers are a special kind of actors that have the power to update synchronized sets.
/// Like agents, they may use information available in any kind of set to do so.
#[pointer_impls(T = U)]
pub trait Synchronizer: Identifiable {
    /// Any errors that this synchronizer can throw during its execution.
    type Error: Error;


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
    ///
    /// # Arguments
    /// - `view`: A runtime [`View`] that represents this synchronizer's view on the current
    ///   system. How partial this view is, and how much is hidden behind the scenes, depends on
    ///   the simulation.
    ///
    /// # Returns
    /// A [`ControlFlow`] which, can either:
    /// - be [`ControlFlow::Continue`], indicating the runtime should continue; or
    /// - a [`ControlFlow::Break`], indicating the system should stop.
    fn poll<T, A, S, E, SM, SA>(&mut self, view: View<T, A, S, E>) -> Result<ControlFlow<()>, Self::Error>
    where
        T: SetMut<Timestamp<T::Timestamp>> + Times,
        A: SetMut<Agreement<SM, T::Timestamp>>,
        S: SetMut<SM>,
        E: SetMut<SA>,
        SM: Message,
        SA: Action;
}
