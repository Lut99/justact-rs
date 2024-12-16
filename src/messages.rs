//  MESSAGES.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 11:43:49
//  Last edited:
//    16 Dec 2024, 15:15:16
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines messages & message sets.
//

use std::collections::HashMap;
use std::convert::Infallible;

use auto_traits::pointer_impls;

use crate::auxillary::{Authored, Identifiable};
use crate::policies::{Extractable, Policy};
use crate::sets::{Set, SetMut};


/***** LIBRARY *****/
/// Defines a single message.
///
/// This is abstract, and now a concrete data structure, because runtimes may wants to decide how
/// they structure the memory of the Message. In particular, messages might be
/// [`Arc`](std::sync::Arc)'ed, and they might want to collide the ID and the author.
#[pointer_impls]
pub trait Message: Authored + Identifiable {
    /// Defines the type of content carried by this message.
    type Payload: Extractable;


    /// Returns the payload of this message.
    ///
    /// # Returns
    /// An immutable reference to the internal [`Message::Payload`].
    fn payload(&self) -> &Self::Payload;
}



/// Defines a bunch of messages.
#[derive(Clone, Debug)]
pub struct MessageSet<M: Identifiable> {
    /// The messages.
    data: HashMap<M::Id, M>,
}

// Constructors
impl<M: Identifiable> Default for MessageSet<M> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl<M: Identifiable> MessageSet<M> {
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
impl<M: Identifiable> Set<M> for MessageSet<M> {
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
    M::Id: Clone,
{
    #[inline]
    fn insert(&mut self, elem: M) -> Result<Option<M>, Self::Error> { Ok(self.data.insert(elem.id().clone(), elem)) }

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
impl<M: Message> Extractable for MessageSet<M> {
    type Policy = <M::Payload as Extractable>::Policy;
    type Error = <M::Payload as Extractable>::Error;


    #[inline]
    fn extract(&self) -> Result<Self::Policy, Self::Error> {
        let mut policy: <M::Payload as Extractable>::Policy = Default::default();
        for msg in self.data.values() {
            policy.compose_mut(msg.payload().extract()?);
        }
        Ok(policy)
    }
}
