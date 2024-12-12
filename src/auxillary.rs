//  AUXILLARY.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 10:54:37
//  Last edited:
//    12 Dec 2024, 13:00:24
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a few small traits for implementing various arrows in the
//!   ontology.
//

use std::hash::Hash;

use crate::times::Timestamp;


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

/// Abstractly defines an object which has an actor.
pub trait Actored {
    /// Some identifier for the actor.
    type ActorId: Eq + Hash;

    /// Returns the ID of the actor of this object.
    ///
    /// # Returns
    /// A reference to an [`Actored::ActorId`] that describes the unique ID of this object's actor.
    fn actor_id(&self) -> &Self::ActorId;
}

// Default impls for pointer-like types.
impl<'a, T: Actored> Actored for &'a T {
    type ActorId = T::ActorId;

    #[inline]
    fn actor_id(&self) -> &Self::ActorId { <T as Actored>::actor_id(self) }
}
impl<'a, T: Actored> Actored for &'a mut T {
    type ActorId = T::ActorId;

    #[inline]
    fn actor_id(&self) -> &Self::ActorId { <T as Actored>::actor_id(self) }
}

/// Abstractly defines an object which has an affector.
pub trait Affectored {
    /// Some identifier for the affector.
    type AffectorId: Eq + Hash;

    /// Returns the ID of the affector of this object.
    ///
    /// # Returns
    /// A reference to an [`Affectored::AffectorId`] that describes the unique ID of this object's
    /// affector.
    fn affector_id(&self) -> &Self::AffectorId;
}

// Default impls for pointer-like types.
impl<'a, T: Affectored> Affectored for &'a T {
    type AffectorId = T::AffectorId;

    #[inline]
    fn affector_id(&self) -> &Self::AffectorId { <T as Affectored>::affector_id(self) }
}
impl<'a, T: Affectored> Affectored for &'a mut T {
    type AffectorId = T::AffectorId;

    #[inline]
    fn affector_id(&self) -> &Self::AffectorId { <T as Affectored>::affector_id(self) }
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



/// Abstractly defines an object which is valid at a certain time.
pub trait Timed {
    /// The representation of the timestamp.
    type Timestamp: Eq + Ord;


    /// Returns the timestamp at which this object was valid.
    ///
    /// # Returns
    /// A [`Timestamp<Timed::Timestamp>`](Timestamp) encoding the timestamp at which it was valid.
    fn at(&self) -> &Timestamp<Self::Timestamp>;
}

// Default impls for pointer-like types.
impl<'a, T: Timed> Timed for &'a T {
    type Timestamp = T::Timestamp;

    #[inline]
    fn at(&self) -> &Timestamp<Self::Timestamp> { <T as Timed>::at(self) }
}
impl<'a, T: Timed> Timed for &'a mut T {
    type Timestamp = T::Timestamp;

    #[inline]
    fn at(&self) -> &Timestamp<Self::Timestamp> { <T as Timed>::at(self) }
}
