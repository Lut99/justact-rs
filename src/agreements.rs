//  AGREEMENTS.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 10:07:55
//  Last edited:
//    21 Jan 2025, 14:59:04
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines agreements, which are like messages but agreed upon by
//!   everybody.
//

use std::convert::Infallible;
use std::hash::Hash;

use crate::auxillary::{Authored, Timed};
use crate::collections::set::Set;


/***** LIBRARY *****/
/// Newtype for a message that everybody agreed upon.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
impl<M: Eq + Hash, T> Set<M> for Agreement<M, T> {
    type Error = Infallible;

    #[inline]
    fn get(&self, elem: &M) -> Result<Option<&M>, Self::Error> { if elem == &self.message { Ok(Some(&self.message)) } else { Ok(None) } }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl 's + Iterator<Item = &'s M>, Self::Error>
    where
        M: 's,
    {
        Ok(Some(&self.message).into_iter())
    }

    #[inline]
    fn len(&self) -> Result<usize, Self::Error> { Ok(1) }
}
impl<M, T: Eq + Ord> Timed for Agreement<M, T> {
    type Timestamp = T;

    #[inline]
    fn at(&self) -> &Self::Timestamp { &self.at }
}
