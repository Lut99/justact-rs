//  AGREEMENTS.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 10:07:55
//  Last edited:
//    11 Dec 2024, 15:21:50
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines agreements, which are like messages but agreed upon by
//!   everybody.
//

use std::hash::Hash;

use crate::auxillary::{Authored, Identifiable, Timed};
use crate::messages::Message;
use crate::policies::Extractable;
use crate::sets::Set;
use crate::times::Timestamp;


/***** LIBRARY *****/
/// Newtype for a message that everybody agreed upon.
#[derive(Clone, Copy, Debug)]
pub struct Agreement<I, A, C, T> {
    /// The message embedded in this agreement.
    pub message: Message<I, A, C>,
    /// The timestamp at which this agreement is valid.
    pub at:      Timestamp<T>,
}

// JustAct
impl<I, A: Eq + Hash, C, T> Authored for Agreement<I, A, C, T> {
    type AuthorId = <Message<I, A, C> as Authored>::AuthorId;

    #[inline]
    fn author_id(&self) -> &Self::AuthorId { self.message.author_id() }
}
impl<I: Eq + Hash, A, C, T> Identifiable for Agreement<I, A, C, T> {
    type Id = <Message<I, A, C> as Identifiable>::Id;

    #[inline]
    fn id(&self) -> &Self::Id { self.message.id() }
}
impl<I: Eq + Hash, A, C, T> Set for Agreement<I, A, C, T> {
    type Elem = <Message<I, A, C> as Set>::Elem;
    type Error = <Message<I, A, C> as Set>::Error;

    #[inline]
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&Self::Elem>, Self::Error> { self.message.get(id) }

    #[inline]
    fn iter(&self) -> Result<impl Iterator<Item = &Self::Elem>, Self::Error> { self.message.iter() }
}
impl<I, A, C: Extractable, T> Extractable for Agreement<I, A, C, T> {
    type Policy = <Message<I, A, C> as Extractable>::Policy;
    type Error = <Message<I, A, C> as Extractable>::Error;


    #[inline]
    fn extract(&self) -> Result<Self::Policy, Self::Error> { self.message.extract() }
}
impl<I, A, C, T: Eq + Ord> Timed for Agreement<I, A, C, T> {
    type Timestamp = T;

    #[inline]
    fn at(&self) -> &Timestamp<Self::Timestamp> { &self.at }
}
