//  AUXILLARY.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 10:54:37
//  Last edited:
//    16 Dec 2024, 14:36:47
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a few small traits for implementing various arrows in the
//!   ontology.
//

use std::hash::Hash;

use auto_traits::pointer_impls;

use crate::times::Timestamp;


/***** LIBRARY *****/
/// Abstractly defines an object which has an author.
#[pointer_impls]
pub trait Authored {
    /// Some identifier for the author.
    type AuthorId: Eq + Hash;

    /// Returns the ID of the author of this object.
    ///
    /// # Returns
    /// A reference to an [`Authored::AuthorId`] that describes the unique ID of this object's author.
    fn author_id(&self) -> &Self::AuthorId;
}

/// Abstractly defines an object which has an actor.
#[pointer_impls]
pub trait Actored {
    /// Some identifier for the actor.
    type ActorId: Eq + Hash;

    /// Returns the ID of the actor of this object.
    ///
    /// # Returns
    /// A reference to an [`Actored::ActorId`] that describes the unique ID of this object's actor.
    fn actor_id(&self) -> &Self::ActorId;
}

/// Abstractly defines an object which has an affector.
#[pointer_impls]
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



/// Abstractly defines an object which is uniquely identifiable by something.
#[pointer_impls]
pub trait Identifiable {
    /// The type of the thing identifying this object.
    type Id: Eq + Hash;

    /// Returns the ID of this object.
    ///
    /// # Returns
    /// A reference to an [`Identifiable::Id`] that describes the unique ID of this object.
    fn id(&self) -> &Self::Id;
}



/// Abstractly defines an object which is valid at a certain time.
#[pointer_impls]
pub trait Timed {
    /// The representation of the timestamp.
    type Timestamp: Eq + Ord;


    /// Returns the timestamp at which this object was valid.
    ///
    /// # Returns
    /// A [`Timestamp<Timed::Timestamp>`](Timestamp) encoding the timestamp at which it was valid.
    fn at(&self) -> &Timestamp<Self::Timestamp>;
}
