//  AGREEMENTS.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 10:07:55
//  Last edited:
//    13 Jan 2025, 16:24:12
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines agreements, which are like messages but agreed upon by
//!   everybody.
//

use std::convert::Infallible;

use crate::auxillary::{Authored, Identifiable, Timed};
use crate::collections::map::Map;


/***** LIBRARY *****/
/// Newtype for a message that everybody agreed upon.
#[derive(Clone, Copy, Debug)]
pub struct Agreement<M, T> {
    /// The message embedded in this agreement.
    pub message: M,
    /// The timestamp at which this agreement is valid.
    pub at:      T,
}

// JustAct
impl<M: Authored, T> Authored for Agreement<M, T> {
    type AuthorId = <M as Authored>::AuthorId;

    #[inline]
    fn author_id(&self) -> &Self::AuthorId { self.message.author_id() }
}
impl<M: Identifiable, T> Identifiable for Agreement<M, T> {
    type Id = <M as Identifiable>::Id;

    #[inline]
    fn id(&self) -> &Self::Id { self.message.id() }
}
impl<M: Identifiable, T> Map<M> for Agreement<M, T> {
    type Error = Infallible;

    #[inline]
    fn get(&self, id: &<M as Identifiable>::Id) -> Result<Option<&M>, Self::Error> {
        if self.message.id() == id { Ok(Some(&self.message)) } else { Ok(None) }
    }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s M>, Self::Error>
    where
        M: 's,
    {
        Ok(Some(&self.message).into_iter())
    }
}
impl<M, T: Eq + Ord> Timed for Agreement<M, T> {
    type Timestamp = T;

    #[inline]
    fn at(&self) -> &Self::Timestamp { &self.at }
}
