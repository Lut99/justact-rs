//  ACTIONS.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 10:06:41
//  Last edited:
//    11 Dec 2024, 15:22:43
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines actions, which enact certain effects.
//

use std::hash::Hash;

use crate::agreements::Agreement;
use crate::auxillary::{Actored, Identifiable, Timed};
use crate::messages::MessageSet;
use crate::times::Timestamp;


/***** LIBRARY *****/
/// Defines an action that an agent can take.
pub struct Action<I, A, C, T> {
    /// The action has its own identifier.
    pub id: I,
    /// The action has an actor.
    pub actor: A,
    /// The basis must always be one of the agreements.
    pub basis: Agreement<I, A, C, T>,
    /// The justication is a bunch of messages satisfying the basis.
    pub justification: MessageSet<I, A, C>,
}

// JustAct impls
impl<I, A: Eq + Hash, C, T> Actored for Action<I, A, C, T> {
    type ActorId = A;

    #[inline]
    fn actor_id(&self) -> &Self::ActorId { &self.actor }
}
impl<I: Eq + Hash, A, C, T> Identifiable for Action<I, A, C, T> {
    type Id = I;

    #[inline]
    fn id(&self) -> &Self::Id { &self.id }
}
impl<I, A, C, T: Eq + Ord> Timed for Action<I, A, C, T> {
    type Timestamp = T;

    #[inline]
    fn at(&self) -> &Timestamp<Self::Timestamp> { &self.basis.at }
}
