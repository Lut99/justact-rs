//  MOD.rs
//    by Lut99
//
//  Created:
//    13 Jan 2025, 16:22:05
//  Last edited:
//    29 Jan 2025, 15:44:46
//  Auto updated?
//    Yes
//
//  Description:
//!   Abstractly defines how sets (and maps) are implemented.
//

// Declare the modules
pub mod map;
pub mod set;

// Imports
use std::convert::Infallible;
use std::ops::{Deref, DerefMut};

use map::Map;
use set::Set;

use crate::auxillary::Identifiable;


/***** LIBRARY *****/
/// Defines ways for agents to choose who to send updates to.
///
/// # Generics
/// - `I`: The type of identifier for the agent in the case of [`Recipient::One`]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Recipient<I> {
    /// Send it to all agents.
    All,
    /// Send it to a specific agent.
    One(I),
}
impl<I> Recipient<I> {
    /// Maps the identifier of the agent, if any.
    ///
    /// # Arguments
    /// - `callback`: Some [`FnOnce`] that will translate `I` to something else. Note it is only
    ///   called if this is a [`Recipient::One`].
    ///
    /// # Returns
    /// A new Recipient with the mapped identifier of an agent.
    #[inline]
    pub fn map<T>(self, callback: impl FnOnce(I) -> T) -> Recipient<T> {
        match self {
            Self::All => Recipient::All,
            Self::One(id) => Recipient::One(callback(id)),
        }
    }
}



/// Defines a singleton set, which has EXACTLY one element (no more, no less).
///
/// Exists to implement read-only [`Set`] and [`Map`] mechanisms for anything that isn't a set.
///
/// Why don't we implement it directly on any `T`, you ask? Well because of a thousand million
/// flippin' "oh maybe someone will implement it again" errors, that's why!
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Singleton<T>(pub T);

// Constructors
impl<T> Singleton<T> {
    /// Constructor for the Singleton.
    ///
    /// This is equivalent to just calling `Singleton(elem)`.
    ///
    /// # Arguments
    /// - `elem`: The element to store in the singleton.
    ///
    /// # Returns
    /// A new [`Singleton`] set that implements [`Set`] and [`Map`].
    #[inline]
    pub const fn new(elem: T) -> Self { Self(elem) }
}

// Deref
impl<T> Deref for Singleton<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for Singleton<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// Sets
impl<T> Set<T> for Singleton<T>
where
    T: PartialEq,
{
    type Error = Infallible;

    #[inline]
    fn contains(&self, elem: &T) -> Result<bool, Self::Error> { Ok(&self.0 == elem) }

    #[inline]
    fn get(&self, elem: &T) -> Result<Option<&T>, Self::Error> { Ok(if &self.0 == elem { Some(&self.0) } else { None }) }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s T>, Self::Error>
    where
        T: 's,
    {
        Ok(Some(&self.0).into_iter())
    }

    #[inline]
    fn len(&self) -> Result<usize, Self::Error> { Ok(1) }
}
impl<T> Map<T> for Singleton<T> {
    type Error = Infallible;

    #[inline]
    fn contains_key(&self, id: &<T as Identifiable>::Id) -> Result<bool, Self::Error>
    where
        T: Identifiable,
    {
        Ok(self.0.id() == id)
    }

    #[inline]
    fn get(&self, id: &<T as Identifiable>::Id) -> Result<Option<&T>, Self::Error>
    where
        T: Identifiable,
    {
        Ok(if self.0.id() == id { Some(&self.0) } else { None })
    }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s T>, Self::Error>
    where
        T: 's + Identifiable,
    {
        Ok(Some(&self.0).into_iter())
    }

    #[inline]
    fn len(&self) -> Result<usize, Self::Error> { Ok(1) }
}

// From
impl<T> From<T> for Singleton<T> {
    #[inline]
    fn from(value: T) -> Self { Self(value) }
}
