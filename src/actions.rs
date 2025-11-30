//  ACTIONS.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 10:06:41
//  Last edited:
//    21 Jan 2025, 14:48:00
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines actions, which enact certain effects.
//

use std::rc::Rc;
use std::sync::Arc;

use auto_traits::pointer_impls;

use crate::agreements::Agreement;
use crate::auxillary::{Actored, Identifiable, Timed};
use crate::messages::MessageSet;


/***** LIBRARY *****/
/// Defines an action that an agent can take.
///
/// Like [`Message`]s, actions are abstract because runtimes may wants to decide how they structure
/// the memory of the Action. In particular, they might want to collide the ID and the author.
#[pointer_impls]
pub trait Action: Actored + Identifiable + Timed {
    /// The type of messages this action uses.
    type Message: Identifiable;


    /// The agreement that forms the basis of the action.
    ///
    /// # Returns
    /// An [`Agreement`] to base the action on.
    fn basis(&self) -> &Agreement<Self::Message, Self::Timestamp>;

    /// Any additional messages that the actor wants to include in the payload of this action.
    ///
    /// "Additional" means beyond the basis & `actor X`-message.
    ///
    /// # Returns
    /// A [`MessageSet`] encoding the extra statements included by the actor.
    fn extra(&self) -> &MessageSet<Self::Message>
    where
        <Self::Message as Identifiable>::Id: ToOwned;

    /// Returns the full payload of this action.
    ///
    /// This is essentially defined as:
    /// ```plain
    /// basis() U extra() U { `actor X.` }
    /// ```
    /// (where `X` is the actor of this action.)
    ///
    /// # Returns
    /// A [`MessageSet`] encoding the extra statements included by the actor.
    fn payload(&self) -> MessageSet<&Self::Message>
    where
        <Self::Message as Identifiable>::Id: ToOwned;
}



/// Defines a constructor for an action.
///
/// This is a more powerful version of an action that can also be constructed, but needn't be one itself.
pub trait ConstructableAction: Clone + Action
where
    Self::Id: ToOwned,
    Self::ActorId: ToOwned,
    <Self::Message as Identifiable>::Id: ToOwned,
{
    /// Constructor for a new action with the given ID, actor, basis and justification.
    ///
    /// # Arguments
    /// - `id`: The identifier of the new actor.
    /// - `actor_id`: The identifier of the action's actor.
    /// - `basis`: The basis used to justify the action.
    /// - `justification`: The justification of the action. Should include the basis!
    ///
    /// # Returns
    /// A new Action.
    fn new(
        id: <Self::Id as ToOwned>::Owned,
        actor_id: <Self::ActorId as ToOwned>::Owned,
        basis: Agreement<Self::Message, Self::Timestamp>,
        justification: MessageSet<Self::Message>,
    ) -> Self
    where
        Self: Sized;
}

// Manual pointer impls (for some of them)
impl<T> ConstructableAction for Box<T>
where
    T: ConstructableAction,
    T::Id: ToOwned,
    T::ActorId: ToOwned,
    <T::Message as Identifiable>::Id: ToOwned,
{
    #[inline]
    fn new(
        id: <Self::Id as ToOwned>::Owned,
        actor_id: <Self::ActorId as ToOwned>::Owned,
        basis: Agreement<Self::Message, Self::Timestamp>,
        justification: MessageSet<Self::Message>,
    ) -> Self
    where
        Self: Sized,
    {
        Box::new(<T as ConstructableAction>::new(id, actor_id, basis, justification))
    }
}
impl<T> ConstructableAction for Rc<T>
where
    T: ConstructableAction,
    T::Id: ToOwned,
    T::ActorId: ToOwned,
    <T::Message as Identifiable>::Id: ToOwned,
{
    #[inline]
    fn new(
        id: <Self::Id as ToOwned>::Owned,
        actor_id: <Self::ActorId as ToOwned>::Owned,
        basis: Agreement<Self::Message, Self::Timestamp>,
        justification: MessageSet<Self::Message>,
    ) -> Self
    where
        Self: Sized,
    {
        Rc::new(<T as ConstructableAction>::new(id, actor_id, basis, justification))
    }
}
impl<T> ConstructableAction for Arc<T>
where
    T: ConstructableAction,
    T::Id: ToOwned,
    T::ActorId: ToOwned,
    <T::Message as Identifiable>::Id: ToOwned,
{
    #[inline]
    fn new(
        id: <Self::Id as ToOwned>::Owned,
        actor_id: <Self::ActorId as ToOwned>::Owned,
        basis: Agreement<Self::Message, Self::Timestamp>,
        justification: MessageSet<Self::Message>,
    ) -> Self
    where
        Self: Sized,
    {
        Arc::new(<T as ConstructableAction>::new(id, actor_id, basis, justification))
    }
}
