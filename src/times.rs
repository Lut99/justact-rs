//  TIMES.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 14:57:21
//  Last edited:
//    11 Dec 2024, 15:29:08
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines timesteps, which are the things that indicate what time it
//!   is.
//

use std::hash::Hash;

use crate::auxillary::{Identifiable, Timed};
use crate::sets::Set;


/***** LIBRARY *****/
/// Defines a single point in time.
///
/// While, conceptually, this is a point in time, for JustAct, it doesn't mean that point is
/// unchanging. This is mostly used to determine when an agreement can be used for new
/// justifications.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timestamp<T> {
    /// The representation of the timestamp.
    pub timestamp: T,
}

// JustAct
impl<T: Eq + Hash> Identifiable for Timestamp<T> {
    type Id = T;

    #[inline]
    fn id(&self) -> &Self::Id { &self.timestamp }
}



/// Extends a [`Set`] of [`Timestamp`]s with functionality to see if an agreement is valid at this
/// timestamp.
pub trait Times: Set<Elem = Timestamp<Self::Timestamp>> {
    /// The timestamp representation used in this set.
    type Timestamp: Eq + Hash + Ord;


    /// Checks if a given timed object is valid at the current time.
    ///
    /// This is used by auditors or other agents at the moment they first receive an enactment to
    /// see if it was done using a current agreement.
    ///
    /// # Arguments
    /// - `obj`: A [`Timed`] object who's time will be checked against the current times.
    ///
    /// # Returns
    /// True if it was valid, or false otherwise.
    ///
    /// # Errors
    /// This function may error at any time the implementation likes.
    fn is_valid(&self, obj: impl Timed) -> Result<bool, Self::Error>;
}
