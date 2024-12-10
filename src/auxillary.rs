//  AUXILLARY.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 10:54:37
//  Last edited:
//    10 Dec 2024, 14:05:58
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a few small traits for implementing various arrows in the
//!   ontology.
//

use std::hash::Hash;


/***** LIBRARY *****/
/// Abstractly defines an object which has an author.
pub trait Authored {
    /// Some identifier for the author.
    type AuthorId: Eq + Hash;

    /// Returns the ID of the author of this object.
    ///
    /// # Returns
    /// A reference to an [`Authored::AuthorId`] that describes the unique ID of this object's author.
    fn author_id(&self) -> &Self::AuthorId;
}

// Default impls for pointer-like types.
impl<'a, T: Authored> Authored for &'a T {
    type AuthorId = T::AuthorId;

    #[inline]
    fn author_id(&self) -> &Self::AuthorId { <T as Authored>::author_id(self) }
}
impl<'a, T: Authored> Authored for &'a mut T {
    type AuthorId = T::AuthorId;

    #[inline]
    fn author_id(&self) -> &Self::AuthorId { <T as Authored>::author_id(self) }
}



/// Abstractly defines an object which is uniquely identifiable by something.
pub trait Identifiable {
    /// The type of the thing identifying this object.
    type Id: Eq + Hash;

    /// Returns the ID of this object.
    ///
    /// # Returns
    /// A reference to an [`Identifiable::Id`] that describes the unique ID of this object.
    fn id(&self) -> &Self::Id;
}

// Default impls for pointer-like types.
impl<'a, T: Identifiable> Identifiable for &'a T {
    type Id = T::Id;

    #[inline]
    fn id(&self) -> &Self::Id { <T as Identifiable>::id(self) }
}
impl<'a, T: Identifiable> Identifiable for &'a mut T {
    type Id = T::Id;

    #[inline]
    fn id(&self) -> &Self::Id { <T as Identifiable>::id(self) }
}
