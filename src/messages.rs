//  MESSAGES.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 11:43:49
//  Last edited:
//    12 Dec 2024, 12:49:04
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines messages & message sets.
//

use std::collections::HashMap;
use std::convert::Infallible;
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::auxillary::{Authored, Identifiable};
use crate::policies::{Extractable, Policy};
use crate::sets::{Set, SetMut};


/***** LIBRARY *****/
/// Defines a single message.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Message<I, A, C> {
    /// The identifier of the message.
    pub id: I,
    /// The author of the message.
    pub author_id: A,
    /// The contents of the message.
    pub contents: C,
}

// Justact impls
impl<I: Eq + Hash, A, C> Identifiable for Message<I, A, C> {
    type Id = I;

    #[inline]
    fn id(&self) -> &Self::Id { &self.id }
}
impl<I, A: Eq + Hash, C> Authored for Message<I, A, C> {
    type AuthorId = A;

    #[inline]
    fn author_id(&self) -> &Self::AuthorId { &self.author_id }
}
impl<I: Eq + Hash, A, C> Set<Self> for Message<I, A, C> {
    type Error = Infallible;

    #[inline]
    fn get(&self, id: &<Self as Identifiable>::Id) -> Result<Option<&Self>, Self::Error> { if &self.id == id { Ok(Some(self)) } else { Ok(None) } }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s Self>, Self::Error>
    where
        Self: 's,
    {
        Ok(Some(self).into_iter())
    }
}
impl<I, A, C: Extractable> Extractable for Message<I, A, C> {
    type Policy = C::Policy;
    type Error = C::Error;


    #[inline]
    fn extract(&self) -> Result<Self::Policy, Self::Error> { <C as Extractable>::extract(&self.contents) }
}



/// Defines a bunch of messages.
#[derive(Clone, Debug)]
pub struct MessageSet<I, A, C> {
    /// The messages.
    data: HashMap<I, Message<I, A, C>>,
}

// Justact impls
impl<I: Eq + Hash, A, C> Set<Message<I, A, C>> for MessageSet<I, A, C> {
    type Error = Infallible;

    #[inline]
    fn get(&self, id: &<Message<I, A, C> as Identifiable>::Id) -> Result<Option<&Message<I, A, C>>, Self::Error> { Ok(self.data.get(id)) }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s Message<I, A, C>>, Self::Error>
    where
        Message<I, A, C>: 's,
    {
        Ok(self.data.values())
    }
}
impl<I: Clone + Eq + Hash, A, C> SetMut<Message<I, A, C>> for MessageSet<I, A, C> {
    #[inline]
    fn insert(&mut self, elem: Message<I, A, C>) -> Result<Option<Message<I, A, C>>, Self::Error> { Ok(self.data.insert(elem.id().clone(), elem)) }

    #[inline]
    fn get_mut(&mut self, id: &<Message<I, A, C> as Identifiable>::Id) -> Result<Option<&mut Message<I, A, C>>, Self::Error> {
        Ok(self.data.get_mut(id))
    }

    #[inline]
    fn remove(&mut self, id: &<Message<I, A, C> as Identifiable>::Id) -> Result<Option<Message<I, A, C>>, Self::Error> { Ok(self.data.remove(id)) }

    #[inline]
    fn iter_mut<'s>(&'s mut self) -> Result<impl Iterator<Item = &'s mut Message<I, A, C>>, Self::Error>
    where
        Message<I, A, C>: 's,
    {
        Ok(self.data.values_mut())
    }
}
impl<I, A, C: Extractable> Extractable for MessageSet<I, A, C> {
    type Policy = C::Policy;
    type Error = C::Error;


    #[inline]
    fn extract(&self) -> Result<Self::Policy, Self::Error> {
        let mut policy: C::Policy = Default::default();
        for msg in self.data.values() {
            policy.compose_mut(msg.contents.extract()?);
        }
        Ok(policy)
    }
}
