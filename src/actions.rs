//  ACTIONS.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 10:06:41
//  Last edited:
//    15 Jan 2025, 10:54:16
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines actions, which enact certain effects.
//

use std::rc::Rc;
use std::sync::Arc;

use crate::agreements::Agreement;
use crate::auxillary::{Actored, Identifiable, Timed};
use crate::messages::MessageSet;


/***** LIBRARY *****/
/// Defines an action that an agent can take.
///
/// Like [`Message`]s, actions are abstract because runtimes may wants to decide how they structure
/// the memory of the Action. In particular, they might want to collide the ID and the author.
pub trait Action: Actored + Identifiable + Timed
where
    Self::Id: ToOwned,
    Self::ActorId: ToOwned,
    <Self::Message as Identifiable>::Id: ToOwned,
{
    /// The type of messages this action uses.
    type Message: Identifiable;


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


    /// The agreement that forms the basis of the action.
    ///
    /// # Returns
    /// An [`Agreement`] to base the action on.
    fn basis(&self) -> &Agreement<Self::Message, Self::Timestamp>;

    /// The justification that should satisfy the agreement.
    ///
    /// Note that this should include the statement embedded by the agreement as well.
    ///
    /// # Returns
    /// A [`MessageSet`] encoding the statements in the justification.
    fn justification(&self) -> &MessageSet<Self::Message>
    where
        <Self::Message as Identifiable>::Id: ToOwned;
}

// Manual pointer impls (for some of them)
impl<T> Action for Box<T>
where
    T: Action,
    T::Id: ToOwned,
    T::ActorId: ToOwned,
    <T::Message as Identifiable>::Id: ToOwned,
{
    type Message = T::Message;

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
        Box::new(<T as Action>::new(id, actor_id, basis, justification))
    }

    #[inline]
    fn basis(&self) -> &Agreement<Self::Message, Self::Timestamp> { <T as Action>::basis(self) }

    #[inline]
    fn justification(&self) -> &MessageSet<Self::Message> { <T as Action>::justification(self) }
}
impl<T> Action for Rc<T>
where
    T: Action,
    T::Id: ToOwned,
    T::ActorId: ToOwned,
    <T::Message as Identifiable>::Id: ToOwned,
{
    type Message = T::Message;

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
        Rc::new(<T as Action>::new(id, actor_id, basis, justification))
    }

    #[inline]
    fn basis(&self) -> &Agreement<Self::Message, Self::Timestamp> { <T as Action>::basis(self) }

    #[inline]
    fn justification(&self) -> &MessageSet<Self::Message> { <T as Action>::justification(self) }
}
impl<T> Action for Arc<T>
where
    T: Action,
    T::Id: ToOwned,
    T::ActorId: ToOwned,
    <T::Message as Identifiable>::Id: ToOwned,
{
    type Message = T::Message;

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
        Arc::new(<T as Action>::new(id, actor_id, basis, justification))
    }

    #[inline]
    fn basis(&self) -> &Agreement<Self::Message, Self::Timestamp> { <T as Action>::basis(self) }

    #[inline]
    fn justification(&self) -> &MessageSet<Self::Message> { <T as Action>::justification(self) }
}
