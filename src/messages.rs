//  MESSAGES.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 11:43:49
//  Last edited:
//    15 Jan 2025, 10:50:59
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines messages & message sets.
//

use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter, Result as FResult};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;

use crate::auxillary::{Authored, Identifiable};
use crate::collections::map::{Map, MapSync};


/***** LIBRARY *****/
/// Defines a single message.
///
/// This is abstract, and not a concrete data structure, because runtimes may wants to decide how
/// they structure the memory of the Message. In particular, messages might be
/// [`Arc`](std::sync::Arc)'ed, and they might want to collide the ID and the author.
pub trait Message: Authored + Identifiable
where
    Self::Id: ToOwned,
    Self::AuthorId: ToOwned,
{
    /// Defines the type of content carried by this message.
    type Payload: ?Sized + ToOwned;


    /// Constructor for a new message with the given ID, author and payload.
    ///
    /// # Arguments
    /// - `id`: The identifier of the new message.
    /// - `author_id`: The identifier of the message's author.
    /// - `payload`: The payload to add to the message.
    ///
    /// # Returns
    /// A new Message.
    fn new(id: <Self::Id as ToOwned>::Owned, author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized;

    /// Returns the payload of this message.
    ///
    /// # Returns
    /// An immutable reference to the internal [`Message::Payload`].
    fn payload(&self) -> &Self::Payload;
}

// Manual pointer impls (for some of them)
impl<T> Message for Box<T>
where
    T: Message,
    T::Id: ToOwned,
    T::AuthorId: ToOwned,
{
    type Payload = T::Payload;

    #[inline]
    fn new(id: <Self::Id as ToOwned>::Owned, author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized,
    {
        Box::new(<T as Message>::new(id, author_id, payload))
    }

    #[inline]
    fn payload(&self) -> &Self::Payload { <T as Message>::payload(self) }
}
impl<T> Message for Rc<T>
where
    T: Message,
    T::Id: ToOwned,
    T::AuthorId: ToOwned,
{
    type Payload = T::Payload;

    #[inline]
    fn new(id: <Self::Id as ToOwned>::Owned, author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized,
    {
        Rc::new(<T as Message>::new(id, author_id, payload))
    }

    #[inline]
    fn payload(&self) -> &Self::Payload { <T as Message>::payload(self) }
}
impl<T> Message for Arc<T>
where
    T: Message,
    T::Id: ToOwned,
    T::AuthorId: ToOwned,
{
    type Payload = T::Payload;

    #[inline]
    fn new(id: <Self::Id as ToOwned>::Owned, author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized,
    {
        Arc::new(<T as Message>::new(id, author_id, payload))
    }

    #[inline]
    fn payload(&self) -> &Self::Payload { <T as Message>::payload(self) }
}



/// Defines a bunch of messages.
pub struct MessageSet<M>
where
    M: Identifiable,
    M::Id: ToOwned,
{
    /// The messages.
    data: HashMap<<M::Id as ToOwned>::Owned, M>,
}

// "Derived" impls
impl<M> Clone for MessageSet<M>
where
    M: Clone + Identifiable,
    M::Id: ToOwned,
    <M::Id as ToOwned>::Owned: Clone,
{
    #[inline]
    fn clone(&self) -> Self { Self { data: self.data.clone() } }
}
impl<M> Debug for MessageSet<M>
where
    M: Debug + Identifiable,
    M::Id: ToOwned,
    <M::Id as ToOwned>::Owned: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        let Self { data } = self;
        let mut fmt = f.debug_struct("MessageSet");
        fmt.field("data", data);
        fmt.finish()
    }
}

// Constructors
impl<M> Default for MessageSet<M>
where
    M: Identifiable,
    M::Id: ToOwned,
{
    #[inline]
    fn default() -> Self { Self::new() }
}
impl<M: Identifiable> MessageSet<M>
where
    M: Identifiable,
    M::Id: ToOwned,
{
    /// Constructor for the MessageSet that initializes it without elements.
    ///
    /// # Returns
    /// A new MessageSet, ready to store messages.
    #[inline]
    pub fn new() -> Self { Self { data: HashMap::new() } }

    /// Constructor for the MessageSet that initializes it without elements, but with the memory
    /// capacity for at least a specified number.
    ///
    /// This is useful for when you are expecting to put some things in there so that you only have
    /// to allocate once.
    ///
    /// # Arguments
    /// - `capacity`: The minimum number of elements that the new set should be able to store
    ///   before having to re-allocate. For optimization/alignment purposes, the actually reserved
    ///   capacity may be higher (see [`HashMap::with_capacity()`]).
    ///
    /// # Returns
    /// A new MessageSet, ready to store messages.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self { Self { data: HashMap::with_capacity(capacity) } }
}

// Justact impls
impl<M> Map<M> for MessageSet<M>
where
    M: Identifiable,
    M::Id: ToOwned,
    <M::Id as ToOwned>::Owned: Eq + Hash,
{
    type Error = Infallible;

    #[inline]
    fn get(&self, id: &<M as Identifiable>::Id) -> Result<Option<&M>, Self::Error> { Ok(self.data.get(id)) }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s M>, Self::Error>
    where
        M: 's,
    {
        Ok(self.data.values())
    }
}
impl<M: Message> MapSync<M> for MessageSet<M>
where
    M: Identifiable,
    M::Id: ToOwned,
    <M::Id as ToOwned>::Owned: Eq + Hash,
    M::AuthorId: ToOwned,
{
    #[inline]
    fn add(&mut self, elem: M) -> Result<Option<M>, Self::Error> { Ok(self.data.insert(elem.id().to_owned(), elem)) }
}

// From
impl<I, M> From<I> for MessageSet<M>
where
    I: IntoIterator<Item = M>,
    M: Identifiable,
    M::Id: ToOwned,
    <M::Id as ToOwned>::Owned: Eq + Hash,
{
    #[inline]
    fn from(value: I) -> Self { MessageSet { data: value.into_iter().map(|m| (m.id().to_owned(), m)).collect() } }
}
impl<M> FromIterator<M> for MessageSet<M>
where
    M: Identifiable,
    M::Id: ToOwned,
    <M::Id as ToOwned>::Owned: Eq + Hash,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = M>>(iter: T) -> Self { Self::from(iter) }
}
