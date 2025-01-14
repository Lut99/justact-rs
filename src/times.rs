//  TIMES.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 14:57:21
//  Last edited:
//    14 Jan 2025, 16:11:58
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines timesteps, which are the things that indicate what time it
//!   is.
//

use std::convert::Infallible;
use std::hash::Hash;

use auto_traits::pointer_impls;

use crate::collections::set::{Set, SetSync};


/***** AUXILLARY *****/
/// Convenience wrapper around [`Times`] for when they are [infallible](std::convert::Infallible).
pub trait InfallibleTimes: Times<Error = Infallible> {
    /// Returns all the current times.
    ///
    /// This is used by auditors or other agents at the moment they first receive an enactment to
    /// see if it was done using a current agreement. I.e., if the action/agreement was at a time
    /// which is in the returned set, it is considered current.
    ///
    /// # Returns
    /// Some set of [`Self::Timestamp`](Times::Timestamp)s that indicate which are current.
    fn current(&self) -> Self::Subset;
}
impl<T: Times<Error = Infallible>> InfallibleTimes for T {
    #[inline]
    fn current(&self) -> Self::Subset {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Times>::current(self).unwrap_unchecked() }
    }
}

/// Convenience wrapper around [`TimesSync`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleTimesSync: TimesSync<Error = Infallible> {
    /// Adds a given timestamp to the current ones.
    ///
    /// This function should always imply also calling [`SetSync::add()`] for the given element.
    /// Remember, after all, that the set of current times needs to be a _subset_ of all times.
    ///
    /// # Arguments
    /// - `timestamp`: A new [`Times::Timestamp`] to mark as current.
    ///
    /// # Returns
    /// True if the element was already marked as current, or false otherwise.
    fn add_current(&mut self, timestamp: Self::Timestamp) -> bool;
}
impl<T: TimesSync<Error = Infallible>> InfallibleTimesSync for T {
    #[inline]
    fn add_current(&mut self, timestamp: Self::Timestamp) -> bool {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as TimesSync>::add_current(self, timestamp).unwrap_unchecked() }
    }
}





/***** LIBRARY *****/
/// Extends a [`Set`] of timestamps with functionality to see which ones are current.
#[pointer_impls]
pub trait Times: Set<Self::Timestamp> {
    /// The type of the returned subset of current times.
    type Subset: Set<Self::Timestamp>;
    /// The timestamp representation used in this set.
    type Timestamp: Eq + Hash + Ord;


    /// Returns all the current times.
    ///
    /// This is used by auditors or other agents at the moment they first receive an enactment to
    /// see if it was done using a current agreement. I.e., if the action/agreement was at a time
    /// which is in the returned set, it is considered current.
    ///
    /// # Returns
    /// Some set of [`Self::Timestamp`](Times::Timestamp)s that indicate which are current.
    ///
    /// # Errors
    /// This function may error at any time the implementation likes.
    fn current(&self) -> Result<Self::Subset, Self::Error>;
}



/// Extends a [`SetSync`] of timestamps with functionality to define which ones are current.
#[pointer_impls]
pub trait TimesSync: SetSync<Self::Timestamp> + Times {
    /// Adds a given timestamp to the current ones.
    ///
    /// This function should always imply also calling [`SetSync::add()`] for the given element.
    /// Remember, after all, that the set of current times needs to be a _subset_ of all times.
    ///
    /// # Arguments
    /// - `timestamp`: A new [`Times::Timestamp`] to mark as current.
    ///
    /// # Returns
    /// True if the element was already marked as current, or false otherwise.
    ///
    /// # Errors
    /// This function may error at any time the implementation likes.
    fn add_current(&mut self, timestamp: Self::Timestamp) -> Result<bool, Self::Error>;
}
