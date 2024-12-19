//  MESSAGES.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 11:43:49
//  Last edited:
//    19 Dec 2024, 11:41:35
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

use auto_traits::pointer_impls;

use crate::auxillary::{Authored, Identifiable};
use crate::sets::{Set, SetMut};


/***** LIBRARY *****/
/// Defines a single message.
///
/// This is abstract, and not a concrete data structure, because runtimes may wants to decide how
/// they structure the memory of the Message. In particular, messages might be
/// [`Arc`](std::sync::Arc)'ed, and they might want to collide the ID and the author.
#[pointer_impls]
pub trait Message: Authored + Identifiable {
    /// Defines the type of content carried by this message.
    type Payload: ?Sized;


    /// Returns the payload of this message.
    ///
    /// # Returns
    /// An immutable reference to the internal [`Message::Payload`].
    fn payload(&self) -> &Self::Payload;
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
impl<M> Set<M> for MessageSet<M>
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
impl<M: Message> SetMut<M> for MessageSet<M>
where
    M: Identifiable,
    M::Id: ToOwned,
    <M::Id as ToOwned>::Owned: Eq + Hash,
{
    #[inline]
    fn insert(&mut self, elem: M) -> Result<Option<M>, Self::Error> { Ok(self.data.insert(elem.id().to_owned(), elem)) }

    #[inline]
    fn get_mut(&mut self, id: &<M as Identifiable>::Id) -> Result<Option<&mut M>, Self::Error> { Ok(self.data.get_mut(id)) }

    #[inline]
    fn remove(&mut self, id: &<M as Identifiable>::Id) -> Result<Option<M>, Self::Error> { Ok(self.data.remove(id)) }

    #[inline]
    fn iter_mut<'s>(&'s mut self) -> Result<impl Iterator<Item = &'s mut M>, Self::Error>
    where
        M: 's,
    {
        Ok(self.data.values_mut())
    }
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
