//  MAP.rs
//    by Lut99
//
//  Created:
//    13 Jan 2025, 16:23:26
//  Last edited:
//    13 Jan 2025, 17:01:39
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines maps, which allow efficient access from identifiers to
//!   elements.
//

use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::hash::Hash;

use auto_traits::pointer_impls;

pub use super::Selector;
use crate::auxillary::Identifiable;


/***** AUXILLARY *****/
/// Convenience wrapper around [`Map`]s for when they are [infallible](std::convert::Infallible).
pub trait InfallibleMap<E>: Map<E, Error = Infallible> {
    /// Checks if a particular element exists in this map.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to check the presence of.
    ///
    /// # Returns
    /// True if such an element is present, or false otherwise.
    fn contains_key(&self, id: &<E as Identifiable>::Id) -> bool
    where
        E: Identifiable;

    /// Retrieves an element with a particular ID from this map.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A read-only reference to the relevant [`Map::Elem`] if an element with `id` existed,
    /// or else [`None`].
    fn get(&self, id: &<E as Identifiable>::Id) -> Option<&E>
    where
        E: Identifiable;

    /// Returns an iterator over the elements in this map.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Map::Elem`] that yields read-only references to every element.
    fn iter<'s>(&'s self) -> impl Iterator<Item = &'s E>
    where
        E: 's + Identifiable;
}
impl<E, T: Map<E, Error = Infallible>> InfallibleMap<E> for T {
    #[inline]
    fn contains_key(&self, id: &<E as Identifiable>::Id) -> bool
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Map<E>>::contains_key(self, id).unwrap_unchecked() }
    }
    #[inline]
    fn get(&self, id: &<E as Identifiable>::Id) -> Option<&E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Map<E>>::get(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn iter<'s>(&'s self) -> impl Iterator<Item = &'s E>
    where
        E: 's + Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Map<E>>::iter(self).unwrap_unchecked() }
    }
}

/// Convenience wrapper around [`MapSync`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleMapSync<E>: Map<E, Error = Infallible> + MapSync<E> {
    /// Adds a new element into the map.
    ///
    /// # Arguments
    /// - `elem`: The [`Map::Elem`] to add to the map.
    ///
    /// # Returns
    /// If an element with the `elem`s ID already existed, it is removed from the map and
    /// returned. Otherwise, if it was new, [`None`] is returned.
    fn add(&mut self, elem: E) -> Option<E>
    where
        E: Identifiable;
}
impl<E, T: Map<E, Error = Infallible> + MapSync<E>> InfallibleMapSync<E> for T {
    #[inline]
    fn add(&mut self, elem: E) -> Option<E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as MapSync<E>>::add(self, elem).unwrap_unchecked() }
    }
}

/// Convenience wrapper around [`MapAsync`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleMapAsync<I, E>: Map<E, Error = Infallible> + MapAsync<I, E>
where
    I: ?Sized,
{
    /// Inserts a new element into the map.
    ///
    /// # Arguments
    /// - `selector`: Some [`Selector`] that can be used to choose who to send the new element to.
    /// - `elem`: The [`Map::Elem`] to add to the set.
    ///
    /// # Returns
    /// If the given element already existed, true is returned. False if it didn't yet.
    fn add(&mut self, selector: Selector<I>, elem: E) -> Option<E>
    where
        E: Identifiable;
}
impl<I, E, T: Map<E, Error = Infallible> + MapAsync<I, E>> InfallibleMapAsync<I, E> for T {
    #[inline]
    fn add(&mut self, selector: Selector<I>, elem: E) -> Option<E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as MapAsync<I, E>>::add(self, selector, elem).unwrap_unchecked() }
    }
}





/***** LIBRARY *****/
/// Defines read-only access to maps.
///
/// This is always how agents can access any kind of map. However, then mutating the map depends on
/// whether it is an [asynchronous](MapAsync) map or a [synchronous](MapSync) one.
#[pointer_impls]
pub trait Map<E> {
    /// The errors potentially thrown when interacting with the map.
    type Error: Error;


    /// Checks if a particular element exists in this map.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to check the presence of.
    ///
    /// # Returns
    /// True if such an element is present, or false otherwise.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent. However, typically, these
    /// are the same conditions as for [`Map::get()`].
    #[inline]
    fn contains_key(&self, id: &<E as Identifiable>::Id) -> Result<bool, Self::Error>
    where
        E: Identifiable,
    {
        self.get(id).map(|r| r.is_some())
    }

    /// Retrieves an element with a particular ID from this map.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A read-only reference to the relevant [`Map::Elem`] if an element with `id` existed,
    /// or else [`None`].
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn get(&self, id: &<E as Identifiable>::Id) -> Result<Option<&E>, Self::Error>
    where
        E: Identifiable;

    /// Returns an iterator over the elements in this map.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Map::Elem`] that yields read-only references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s E>, Self::Error>
    where
        E: 's + Identifiable;
}

// Default impls for std types.
impl<T: Identifiable> Map<T> for Vec<T> {
    type Error = Infallible;


    #[inline]
    fn get(&self, id: &<T as Identifiable>::Id) -> Result<Option<&T>, Self::Error>
    where
        T: Identifiable,
    {
        for elem in self {
            if elem.id() == id {
                return Ok(Some(elem));
            }
        }
        Ok(None)
    }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s T>, Self::Error>
    where
        T: 's + Identifiable,
    {
        Ok(<[T]>::iter(self))
    }
}
impl<T: Identifiable> Map<T> for HashMap<<T::Id as ToOwned>::Owned, T>
where
    T: Identifiable,
    T::Id: ToOwned,
    <T::Id as ToOwned>::Owned: Eq + Hash,
{
    type Error = Infallible;


    #[inline]
    fn get(&self, id: &<T as Identifiable>::Id) -> Result<Option<&T>, Self::Error>
    where
        T: Identifiable,
    {
        Ok(HashMap::get(self, id))
    }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s T>, Self::Error>
    where
        T: 's + Identifiable,
    {
        Ok(<Self>::values(self))
    }
}



/// Defines mutable access to synchronous maps.
///
/// Agents can always [access maps immutably](Map). However, mutable access is determined by how an
/// agent can get access to it. This trait defines it in the case that the view of all agents on a
/// map is the same; i.e., it is impossible for agents to choose whom to send their updates to.
#[pointer_impls]
pub trait MapSync<E>: Map<E> {
    /// Adds a new element into the map.
    ///
    /// # Arguments
    /// - `elem`: The [`Map::Elem`] to add to the map.
    ///
    /// # Returns
    /// If an element with the `elem`s ID already existed, it is removed from the map and
    /// returned. Otherwise, if it was new, [`None`] is returned.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn add(&mut self, elem: E) -> Result<Option<E>, Self::Error>
    where
        E: Identifiable;
}

// Default impls for std types.
impl<T: Identifiable> MapSync<T> for Vec<T> {
    #[inline]
    fn add(&mut self, mut new_elem: T) -> Result<Option<T>, Self::Error>
    where
        T: Identifiable,
    {
        let id: &T::Id = new_elem.id();
        for elem in <[T]>::iter_mut(self) {
            if id == elem.id() {
                std::mem::swap(&mut new_elem, elem);
                return Ok(Some(new_elem));
            }
        }
        self.push(new_elem);
        Ok(None)
    }
}
impl<T> MapSync<T> for HashMap<<T::Id as ToOwned>::Owned, T>
where
    T: Identifiable,
    T::Id: ToOwned,
    <T::Id as ToOwned>::Owned: Eq + Hash,
{
    #[inline]
    fn add(&mut self, elem: T) -> Result<Option<T>, Self::Error>
    where
        T: Identifiable,
    {
        Ok(<Self>::insert(self, elem.id().to_owned(), elem))
    }
}



/// Defines mutable access to asynchronous maps.
///
/// Agents can always [access maps immutably](Map). However, mutable access is determined by how an
/// agent can get access to it. This trait defines it in the case that the view of agents on a map
/// differs: agents get to choose who they send their updates to.
#[pointer_impls]
pub trait MapAsync<I, E>: Map<E>
where
    I: ?Sized,
{
    /// Inserts a new element into the map.
    ///
    /// # Arguments
    /// - `selector`: Some [`Selector`] that can be used to choose who to send the new element to.
    /// - `elem`: The [`Map::Elem`] to add to the set.
    ///
    /// # Returns
    /// If the given element already existed, true is returned. False if it didn't yet.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn add(&mut self, selector: Selector<I>, elem: E) -> Result<Option<E>, Self::Error>
    where
        E: Identifiable;
}
