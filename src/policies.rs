//  POLICIES.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 12:00:42
//  Last edited:
//    23 Jan 2025, 16:55:22
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the interface with policy in JustAct.
//

use std::error::Error;

use auto_traits::pointer_impls;

use crate::auxillary::{Affectored, Identifiable};
use crate::collections::map::{InfallibleMap, Map};
use crate::collections::set::InfallibleSet;
use crate::messages::Message;


/***** LIBRARY *****/
/// Defines how a single effect in the policy's [`Denotation`] looks like.
///
/// Effects are like truths, but have an additional effector agent that does
/// them. As such, they are also identified by facts.
#[pointer_impls]
pub trait Effect: Affectored + Identifiable {
    type Fact;

    /// Returns the fact that encodes the effect.
    ///
    /// # Returns
    /// The fact that is effected.
    fn fact(&self) -> &Self::Fact;
}

/// Defines how the interpretation of a snippet of policy looks like.
///
/// The Denotation is viewed as a set over _truths_: i.e., when iterating over it, only facts that
/// are TRUE are yielded, not facts that are known to be false.
#[pointer_impls]
pub trait Denotation: InfallibleMap<Self::Effect> + InfallibleSet<Self::Fact> {
    /// The shape of effects that can be inferred from a policy.
    type Effect: Effect;
    /// The shape of truth that can be inferred from a policy.
    type Fact;


    /// Checks whether a given fact is true in this denotation.
    ///
    /// Note that this is done under the closed world assumption. I.e., the absence of truth equals
    /// false.
    ///
    /// # Arguments
    /// - `fact`: Some [`Denotation::Fact`] of which we want to learn the truth.
    ///
    /// # Returns
    /// [`Some(true)`] if the fact was known to be true, or [`Some(false)`] otherwise.
    ///
    /// Some semantics may define a third value, [`None`], which encodes that the value is
    /// _unknowable_ (not just unknown). An example of this is a logical contradiction.
    fn truth_of(&self, fact: &Self::Fact) -> Option<bool>;


    /// Returns an iterator over the truths in this [`Denotation`].
    ///
    /// By default, this is an alias for the [`InfallibleSet::<Self::Fact>::iter()`](InfallibleSet::iter()) implementation.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Denotatation::Fact`]s.
    #[inline]
    fn iter_truths<'s>(&'s self) -> impl 's + Iterator<Item = &'s Self::Fact> { <Self as InfallibleSet<Self::Fact>>::iter(self) }

    /// Returns an iterator over the effects in this [`Denotation`].
    ///
    /// By default, this is an alias for the [`InfallibleMap::<Self::Effect>::iter()`](InfallibleMap::iter()) implementation.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Denotatation::Effect`]s.
    #[inline]
    fn iter_effects<'s>(&'s self) -> impl 's + Iterator<Item = &'s Self::Effect> { <Self as InfallibleMap<Self::Effect>>::iter(self) }
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
///
/// # Generics
/// - `'m`: The lifetime of the message(s) extracted from in case the policy needs it.
/// - `A`: The type of agent identifiers that this extractor is compatible with.
/// - `P`: The type of message payloads that this extractor is compatible with.
#[pointer_impls]
pub trait Extractor<'m, A, C>
where
    A: ?Sized + ToOwned,
    C: ?Sized,
{
    /// The policy extracted.
    type Policy: Policy;
    /// Any errors thrown if the policy in this object is unparseable.
    type Error: Error;


    /// Extracts the policy from something iterating over messages.
    ///
    /// # Arguments
    /// - `msgs`: A [`Map`] of messages that we will be extracting from.
    ///
    /// # Returns
    /// An [`Extractable::Policy`] that describes the policy extracted from `msgs`.
    ///
    /// # Errors
    /// This function should error if and only if the policy contained in this object fails to
    /// parse.
    fn extract<M: Message<AuthorId = A, Payload = C>>(&self, msgs: &'m impl Map<M>) -> Result<Self::Policy, Self::Error>;
}
