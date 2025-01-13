//  COLLECTIONS.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 10:40:27
//  Last edited:
//    13 Jan 2025, 14:43:18
//  Auto updated?
//    Yes
//
//  Description:
//!   Abstractly defines how sets and maps are implemented.
//

use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::error::Error;
use std::hash::Hash;

use auto_traits::pointer_impls;

use crate::auxillary::Identifiable;


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
    fn iter<'s>(&'s self) -> impl Iterator<Item = &'s E>
    where
        E: 's;
}

// Default impls
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
    fn iter<'s>(&'s self) -> impl Iterator<Item = &'s E>
    where
        E: 's,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set<E>>::iter(self).unwrap_unchecked() }
    }
}



/// Convenience wrapper around [`SetMut`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleSetMut<E>: Set<E, Error = Infallible> + SetMut<E> {
    /// Inserts a new element into the set.
    ///
    /// # Arguments
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Returns
    /// If the given element already existed, true is returned. False if it didn't yet.
    fn insert(&mut self, elem: E) -> bool;

    /// Removes an element with a particular ID from this set and returns it.
    ///
    /// # Arguments
    /// - `elem`: The element to retrieve.
    ///
    /// # Returns
    /// True if the element existed, or else false.
    fn remove(&mut self, elem: &E) -> bool;
}

// Default impls
impl<E, T: Set<E, Error = Infallible> + SetMut<E>> InfallibleSetMut<E> for T {
    #[inline]
    fn insert(&mut self, elem: E) -> bool {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut<E>>::insert(self, elem).unwrap_unchecked() }
    }

    #[inline]
    fn remove(&mut self, elem: &E) -> bool {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut<E>>::remove(self, elem).unwrap_unchecked() }
    }
}



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

// Default impls
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



/// Convenience wrapper around [`MapMut`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleMapMut<E>: Map<E, Error = Infallible> + MapMut<E> {
    /// Inserts a new element into the map.
    ///
    /// # Arguments
    /// - `elem`: The [`Map::Elem`] to add to the map.
    ///
    /// # Returns
    /// If an element with the `elem`s ID already existed, it is removed from the map and
    /// returned. Otherwise, if it was new, [`None`] is returned.
    fn insert(&mut self, elem: E) -> Option<E>
    where
        E: Identifiable;



    /// Retrieves an element with a particular ID from this map such that it is mutable.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A mutable reference to the relevant [`Map::Elem`] if an element with `id` existed,
    /// or else [`None`].
    fn get_mut(&mut self, id: &<E as Identifiable>::Id) -> Option<&mut E>
    where
        E: Identifiable;

    /// Removes an element with a particular ID from this map and returns it.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// The relevant [`Map::Elem`] if an element with `id` existed, or else
    /// [`None`].
    fn remove(&mut self, id: &<E as Identifiable>::Id) -> Option<E>
    where
        E: Identifiable;

    /// Returns a mutable iterator over the elements in this map.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Map::Elem`] that yields mutable references to every element.
    fn iter_mut<'s>(&'s mut self) -> impl Iterator<Item = &'s mut E>
    where
        E: 's + Identifiable;
}

// Default impls
impl<E, T: Map<E, Error = Infallible> + MapMut<E>> InfallibleMapMut<E> for T {
    #[inline]
    fn insert(&mut self, elem: E) -> Option<E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as MapMut<E>>::insert(self, elem).unwrap_unchecked() }
    }

    #[inline]
    fn get_mut(&mut self, id: &<E as Identifiable>::Id) -> Option<&mut E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as MapMut<E>>::get_mut(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn remove(&mut self, id: &<E as Identifiable>::Id) -> Option<E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as MapMut<E>>::remove(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn iter_mut<'s>(&'s mut self) -> impl Iterator<Item = &'s mut E>
    where
        E: 's + Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as MapMut<E>>::iter_mut(self).unwrap_unchecked() }
    }
}





/***** LIBRARY *****/
/// Defines read-only access to sets.
///
/// This is always how agents can access sets. However, they can only
/// [interface with a set mutably](SetMut) if that set is synchronous.
#[pointer_impls]
pub trait Set<E> {
    /// The errors potentially thrown when interacting with the map.
    type Error: Error;


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
    fn contains(&self, id: &E) -> Result<bool, Self::Error> { self.get(id).map(|id| id.is_some()) }

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
    fn get(&self, id: &E) -> Result<Option<&E>, Self::Error>;

    /// Returns an iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields read-only references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s E>, Self::Error>
    where
        E: 's;
}

// Default impls for std types.
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
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s T>, Self::Error>
    where
        T: 's,
    {
        Ok(<[T]>::iter(self))
    }
}
impl<T> Set<T> for HashSet<T>
where
    T: Eq + Hash,
{
    type Error = Infallible;


    #[inline]
    fn get(&self, elem: &T) -> Result<Option<&T>, Self::Error> { Ok(HashSet::get(self, elem)) }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s T>, Self::Error>
    where
        T: 's,
    {
        Ok(<Self>::iter(self))
    }
}



/// Defines mutable access to sets.
///
/// Agents can always [access sets immutably](Map). However, they can only do so mutably if that
/// map is synchronous.
#[pointer_impls]
pub trait SetMut<E>: Set<E> {
    /// Inserts a new element into the set.
    ///
    /// # Arguments
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Returns
    /// If the given element already existed, true is returned. False if it didn't yet.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn insert(&mut self, elem: E) -> Result<bool, Self::Error>;

    /// Removes an element with a particular ID from this set and returns it.
    ///
    /// # Arguments
    /// - `elem`: The element to retrieve.
    ///
    /// # Returns
    /// True if the element existed, or else false.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn remove(&mut self, elem: &E) -> Result<bool, Self::Error>;
}

// Default impls for std types.
impl<T> SetMut<T> for Vec<T>
where
    T: PartialEq,
{
    #[inline]
    fn insert(&mut self, mut new_elem: T) -> Result<bool, Self::Error> {
        for elem in <[T]>::iter_mut(self) {
            if elem == &mut new_elem {
                std::mem::swap(&mut new_elem, elem);
                return Ok(true);
            }
        }
        self.push(new_elem);
        Ok(false)
    }

    #[inline]
    fn remove(&mut self, new_elem: &T) -> Result<bool, Self::Error> {
        for (i, elem) in <[T]>::iter_mut(self).enumerate() {
            if elem == new_elem {
                self.swap_remove(i);
                return Ok(true);
            }
        }
        Ok(false)
    }
}
impl<T> SetMut<T> for HashSet<T>
where
    T: Eq + Hash,
{
    #[inline]
    fn insert(&mut self, elem: T) -> Result<bool, Self::Error> { Ok(<Self>::insert(self, elem)) }

    #[inline]
    fn remove(&mut self, elem: &T) -> Result<bool, Self::Error> { Ok(<Self>::remove(self, elem)) }
}



/// Defines read-only access to maps.
///
/// This is always how agents can access maps. However, they can only
/// [interface with a map mutably](MapMut) if that map is synchronous.
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



/// Defines mutable access to maps.
///
/// Agents can always [access maps immutably](Map). However, they can only do so mutably if that
/// map is synchronous.
#[pointer_impls]
pub trait MapMut<E>: Map<E> {
    /// Inserts a new element into the map.
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
    fn insert(&mut self, elem: E) -> Result<Option<E>, Self::Error>
    where
        E: Identifiable;



    /// Retrieves an element with a particular ID from this map such that it is mutable.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A mutable reference to the relevant [`Map::Elem`] if an element with `id` existed,
    /// or else [`None`].
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn get_mut(&mut self, id: &<E as Identifiable>::Id) -> Result<Option<&mut E>, Self::Error>
    where
        E: Identifiable;

    /// Removes an element with a particular ID from this map and returns it.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// The relevant [`Map::Elem`] if an element with `id` existed, or else
    /// [`None`].
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn remove(&mut self, id: &<E as Identifiable>::Id) -> Result<Option<E>, Self::Error>
    where
        E: Identifiable;

    /// Returns a mutable iterator over the elements in this map.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Map::Elem`] that yields mutable references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter_mut<'s>(&'s mut self) -> Result<impl Iterator<Item = &'s mut E>, Self::Error>
    where
        E: 's + Identifiable;
}

// Default impls for std types.
impl<T: Identifiable> MapMut<T> for Vec<T> {
    #[inline]
    fn insert(&mut self, mut new_elem: T) -> Result<Option<T>, Self::Error>
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

    #[inline]
    fn get_mut(&mut self, id: &<T as Identifiable>::Id) -> Result<Option<&mut T>, Self::Error>
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
    fn remove(&mut self, id: &<T as Identifiable>::Id) -> Result<Option<T>, Self::Error>
    where
        T: Identifiable,
    {
        for (i, elem) in <[T]>::iter_mut(self).enumerate() {
            if elem.id() == id {
                return Ok(Some(self.swap_remove(i)));
            }
        }
        Ok(None)
    }

    #[inline]
    fn iter_mut<'s>(&'s mut self) -> Result<impl Iterator<Item = &'s mut T>, Self::Error>
    where
        T: 's + Identifiable,
    {
        Ok(<[T]>::iter_mut(self))
    }
}
impl<T> MapMut<T> for HashMap<<T::Id as ToOwned>::Owned, T>
where
    T: Identifiable,
    T::Id: ToOwned,
    <T::Id as ToOwned>::Owned: Eq + Hash,
{
    #[inline]
    fn insert(&mut self, elem: T) -> Result<Option<T>, Self::Error>
    where
        T: Identifiable,
    {
        Ok(<Self>::insert(self, elem.id().to_owned(), elem))
    }

    #[inline]
    fn get_mut(&mut self, id: &<T as Identifiable>::Id) -> Result<Option<&mut T>, Self::Error>
    where
        T: Identifiable,
    {
        Ok(<Self>::get_mut(self, id))
    }

    #[inline]
    fn remove(&mut self, id: &<T as Identifiable>::Id) -> Result<Option<T>, Self::Error>
    where
        T: Identifiable,
    {
        Ok(<Self>::remove(self, id))
    }

    #[inline]
    fn iter_mut<'s>(&'s mut self) -> Result<impl Iterator<Item = &'s mut T>, Self::Error>
    where
        T: 's + Identifiable,
    {
        Ok(<Self>::values_mut(self))
    }
}
