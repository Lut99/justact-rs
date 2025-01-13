//  TIMES.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 14:57:21
//  Last edited:
//    13 Jan 2025, 14:25:14
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines timesteps, which are the things that indicate what time it
//!   is.
//

use std::hash::Hash;

use auto_traits::pointer_impls;

use crate::auxillary::Timed;
use crate::collections::Set;


/***** LIBRARY *****/
/// Extends a [`Set`] of [`Timestamp`]s with functionality to see if an agreement is valid at this
/// timestamp.
#[pointer_impls]
pub trait Times: Set<Self::Timestamp> {
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
