//  POLICIES.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 12:00:42
//  Last edited:
//    16 Dec 2024, 15:28:32
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the interface with policy in JustAct.
//

use std::error::Error;

use auto_traits::pointer_impls;

use crate::auxillary::{Affectored, Identifiable};
use crate::sets::InfallibleSet;


/***** LIBRARY *****/
/// Defines how a single effect in the policy's [`Denotation`] looks like.
///
/// Effects are like truths, but have an additional effector agent that does
/// them. As such, they are also identified by facts.
#[pointer_impls]
pub trait Effect: Affectored + Truth {}

/// Defines how a single truth in the policy's [`Denotation`] looks like.
///
/// Truths are identifiable by facts. This is represented by
/// [`<Truth as Identifiable>::Id`](Identifiable::Id).
#[pointer_impls]
pub trait Truth: Identifiable {
    /// Returns the fact that is truth'ed by this Truth.
    ///
    /// By default, this function is a semantically more meaningful alias for
    /// [`<Truth as Identifiable>::id()`](Identifiable::id()).
    ///
    /// # Returns
    /// The fact that is truth'ed by this Truth.
    #[inline]
    fn fact(&self) -> &Self::Id { <Self as Identifiable>::id(self) }

    /// Returns the inner value of this Truth.
    ///
    /// Note that this is done under the closed world assumption. I.e., the absence of truth equals
    /// false.
    ///
    /// # Returns
    /// True if the fact holds, or false otherwise.
    ///
    /// In some semantics, logical conflicts collapse to a special error or unknown value. This can
    /// be communicated by returning [`None`].
    fn value(&self) -> Option<bool>;
}

/// Defines how the interpretation of a snippet of policy looks like.
#[pointer_impls]
pub trait Denotation: InfallibleSet<Self::Effect> + InfallibleSet<Self::Truth> {
    /// The shape of effects that can be inferred from a policy.
    type Effect: Effect;
    /// The shape of truth that can be inferred from a policy.
    type Truth: Truth;


    /// Checks whether a given fact is true in this denotation.
    ///
    /// Note that this is done under the closed world assumption. I.e., the absence of truth equals
    /// false.
    ///
    /// The default implementation simply wraps [`Truth::value()`] if the given fact is in the
    /// denotation. Else, it assumes `Some(false)`.
    ///
    /// # Arguments
    /// - `fact`: Some [`Denotation::Fact`] of which we want to learn the truth.
    ///
    /// # Returns
    /// A [`Self::Truth`]
    ///
    /// In some semantics, logical conflicts collapse to a special error or unknown value. This can
    /// be communicated by returning [`None`].
    fn truth_of(&self, fact: &<Self::Truth as Identifiable>::Id) -> Option<bool> {
        <Self as InfallibleSet<Self::Truth>>::get(self, fact).and_then(Self::Truth::value).or_else(|| Some(false))
    }
}



/// Defines how policy looks like once extracted.
pub trait Policy: Default {
    /// Describes the truths that currently hold in the policy.
    type Denotation: Denotation;


    /// Computes the validity of the policy.
    ///
    /// # Returns
    /// True if the policy is valid, false otherwise.
    fn is_valid(&self) -> bool;

    /// Computes the denotation of the policy.
    ///
    /// This will return a set of all the truths computed by the policy.
    ///
    /// # Returns
    /// A [`Policy::Denotation`] that describes the active truths in the policy.
    fn truths(&self) -> Self::Denotation;



    /// Composes a grander set of policy from this policy.
    ///
    /// # Arguments
    /// - `other`: Some other policy snippet to compose with.
    ///
    /// # Returns
    /// A new snippet of policy that composes both `self` and `other`.
    fn compose(&self, other: Self) -> Self;

    /// Extends this policy by composing another snippet into it.
    ///
    /// # Arguments
    /// - `other`: Some other policy snippet to compose with.
    fn compose_mut(&mut self, other: Self);
}



/// Defines that something can extract policy.
#[pointer_impls]
pub trait Extractable {
    /// The policy extracted.
    type Policy: Policy;
    /// Any errors thrown if the policy in this object is unparseable.
    type Error: Error;


    /// Extracts the policy from this object.
    ///
    /// # Returns
    /// An [`Extractable::Policy`] that describes the policy in this object.
    ///
    /// # Errors
    /// This function should error if and only if the policy contained in this object fails to
    /// parse.
    fn extract(&self) -> Result<Self::Policy, Self::Error>;
}
