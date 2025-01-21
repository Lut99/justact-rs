//  SET.rs
//    by Lut99
//
//  Created:
//    13 Jan 2025, 16:22:42
//  Last edited:
//    21 Jan 2025, 14:56:14
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements sets, i.e., maps that use the elements themselves for
//!   identifications.
//

use std::collections::HashSet;
use std::convert::Infallible;
use std::error::Error;
use std::hash::Hash;

use auto_traits::pointer_impls;

pub use super::Selector;


/***** AUXILLARY *****/
/// Convenience wrapper around [`Set`]s for when they are [infallible](std::convert::Infallible).
pub trait InfallibleSet<E>: Set<E, Error = Infallible> {
    /// Checks if a particular element exists in this set.
    ///
    /// # Arguments
    /// - `elem`: The element to check the presence of.
    ///
    /// # Returns
    /// True if the element is present, or false otherwise.
    fn contains(&self, elem: &E) -> bool;

    /// Retrieves a particular element from this set.
    ///
    /// While this function may seem redundant (the `elem` already exists and given as input), this
    /// can be used to refer to an instance of it with a different lifetime (that of the set).
    ///
    /// # Arguments
    /// - `elem`: The element to check the presence of.
    ///
    /// # Returns
    /// A read-only reference to the relevant [`Set::Elem`] if it existed, or [`None`] otherwise.
    fn get(&self, id: &E) -> Option<&E>;

    /// Returns an iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields read-only references to every element.
    fn iter<'s>(&'s self) -> impl 's + Iterator<Item = &'s E>
    where
        E: 's;

    /// Returns how many elements there are in this set.
    ///
    /// # Returns
    /// A [`usize`] encoding this.
    fn len(&self) -> usize;
}
impl<E, T: Set<E, Error = Infallible>> InfallibleSet<E> for T {
    #[inline]
    fn contains(&self, elem: &E) -> bool {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set<E>>::contains(self, elem).unwrap_unchecked() }
    }

    #[inline]
    fn get(&self, elem: &E) -> Option<&E> {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set<E>>::get(self, elem).unwrap_unchecked() }
    }

    #[inline]
    fn iter<'s>(&'s self) -> impl 's + Iterator<Item = &'s E>
    where
        E: 's,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set<E>>::iter(self).unwrap_unchecked() }
    }

    #[inline]
    fn len(&self) -> usize {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set<E>>::len(self).unwrap_unchecked() }
    }
}

/// Convenience wrapper around [`SetSync`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleSetSync<E>: Set<E, Error = Infallible> + SetSync<E> {
    /// Adds a new element into the set.
    ///
    /// # Arguments
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Returns
    /// If the given element already existed, true is returned. False if it didn't yet.
    fn add(&mut self, elem: E) -> bool;
}
impl<E, T: Set<E, Error = Infallible> + SetSync<E>> InfallibleSetSync<E> for T {
    #[inline]
    fn add(&mut self, elem: E) -> bool {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetSync<E>>::add(self, elem).unwrap_unchecked() }
    }
}

/// Convenience wrapper around [`SetAsync`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleSetAsync<I, E>: Set<E, Error = Infallible> + SetAsync<I, E>
where
    I: ?Sized,
{
    /// Inserts a new element into the set.
    ///
    /// # Arguments
    /// - `selector`: Some [`Selector`] that can be used to choose who to send the new element to.
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Returns
    /// If the given element already existed, true is returned. False if it didn't yet.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn add(&mut self, selector: Selector<&I>, elem: E);
}
impl<I, E, T: Set<E, Error = Infallible> + SetAsync<I, E>> InfallibleSetAsync<I, E> for T {
    #[inline]
    fn add(&mut self, selector: Selector<&I>, elem: E) {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetAsync<I, E>>::add(self, selector, elem).unwrap_unchecked() }
    }
}





/***** LIBRARY *****/
/// Defines read-only access to sets.
///
/// This is always how agents can access any kind of set. However, then mutating the set depends on
/// whether it is an [asynchronous](SetAsync) set or a [synchronous one](SetSync).
#[pointer_impls]
pub trait Set<E> {
    /// The errors potentially thrown when interacting with the map.
    type Error: 'static + Error;


    /// Checks if a particular element exists in this set.
    ///
    /// # Arguments
    /// - `elem`: The element to check the presence of.
    ///
    /// # Returns
    /// True if the element is present, or false otherwise.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent. However, typically, these
    /// are the same conditions as for [`Set::get()`].
    #[inline]
    fn contains(&self, elem: &E) -> Result<bool, Self::Error> { self.get(elem).map(|elem| elem.is_some()) }

    /// Retrieves a particular element from this set.
    ///
    /// While this function may seem redundant (the `elem` already exists and given as input), this
    /// can be used to refer to an instance of it with a different lifetime (that of the set).
    ///
    /// # Arguments
    /// - `elem`: The element to check the presence of.
    ///
    /// # Returns
    /// A read-only reference to the relevant [`Set::Elem`] if it existed, or [`None`] otherwise.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn get(&self, elem: &E) -> Result<Option<&E>, Self::Error>;

    /// Returns an iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields read-only references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter<'s>(&'s self) -> Result<impl 's + Iterator<Item = &'s E>, Self::Error>
    where
        E: 's;

    /// Returns how many elements there are in this set.
    ///
    /// # Returns
    /// A [`usize`] encoding this.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn len(&self) -> Result<usize, Self::Error>;
}

// Default impls for std types.
impl<T> Set<T> for Option<T>
where
    T: PartialEq,
{
    type Error = Infallible;

    #[inline]
    fn get(&self, elem: &T) -> Result<Option<&T>, Self::Error> { Ok(self.as_ref().filter(|s| *s == elem)) }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl 's + Iterator<Item = &'s T>, Self::Error>
    where
        T: 's,
    {
        Ok(self.as_ref().into_iter())
    }

    #[inline]
    fn len(&self) -> Result<usize, Self::Error> { Ok(if self.is_some() { 1 } else { 0 }) }
}
impl<T> Set<T> for Vec<T>
where
    T: PartialEq,
{
    type Error = Infallible;


    #[inline]
    fn get(&self, new_elem: &T) -> Result<Option<&T>, Self::Error> {
        for elem in self {
            if elem == new_elem {
                return Ok(Some(elem));
            }
        }
        Ok(None)
    }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl 's + Iterator<Item = &'s T>, Self::Error>
    where
        T: 's,
    {
        Ok(<[T]>::iter(self))
    }

    #[inline]
    fn len(&self) -> Result<usize, Self::Error> { Ok(<Self>::len(self)) }
}
impl<T> Set<T> for HashSet<T>
where
    T: Eq + Hash,
{
    type Error = Infallible;


    #[inline]
    fn get(&self, elem: &T) -> Result<Option<&T>, Self::Error> { Ok(HashSet::get(self, elem)) }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl 's + Iterator<Item = &'s T>, Self::Error>
    where
        T: 's,
    {
        Ok(<Self>::iter(self))
    }

    #[inline]
    fn len(&self) -> Result<usize, Self::Error> { Ok(<Self>::len(self)) }
}



/// Defines mutable access to synchronous sets.
///
/// Agents can always [access sets immutably](Set). However, mutable access is determined by how an
/// agent can get access to it. This trait defines it in the case that the view of all agents on a
/// set is the same; i.e., it is impossible for agents to choose whom to send their updates to.
#[pointer_impls]
pub trait SetSync<E>: Set<E> {
    /// Adds a new element into the set.
    ///
    /// # Arguments
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Returns
    /// If the given element already existed, true is returned. False if it didn't yet.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn add(&mut self, elem: E) -> Result<bool, Self::Error>;
}

// Default impls for std types.
impl<T> SetSync<T> for Vec<T>
where
    T: PartialEq,
{
    #[inline]
    fn add(&mut self, mut new_elem: T) -> Result<bool, Self::Error> {
        for elem in <[T]>::iter_mut(self) {
            if elem == &mut new_elem {
                std::mem::swap(&mut new_elem, elem);
                return Ok(true);
            }
        }
        self.push(new_elem);
        Ok(false)
    }
}
impl<T> SetSync<T> for HashSet<T>
where
    T: Eq + Hash,
{
    #[inline]
    fn add(&mut self, elem: T) -> Result<bool, Self::Error> { Ok(<Self>::insert(self, elem)) }
}



/// Defines mutable access to asynchronous sets.
///
/// Agents can always [access sets immutably](Set). However, mutable access is determined by how an
/// agent can get access to it. This trait defines it in the case that the view of agents on a set
/// differs: agents get to choose who they send their updates to.
#[pointer_impls]
pub trait SetAsync<I, E>: Set<E>
where
    I: ?Sized,
{
    /// Inserts a new element into the set.
    ///
    /// # Arguments
    /// - `selector`: Some [`Selector`] that can be used to choose who to send the new element to.
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn add(&mut self, selector: Selector<&I>, elem: E) -> Result<(), Self::Error>;
}
