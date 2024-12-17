//  SETS.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 10:40:27
//  Last edited:
//    17 Dec 2024, 15:41:17
//  Auto updated?
//    Yes
//
//  Description:
//!   Abstractly defines how a set is implemented.
//

use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::hash::Hash;

use auto_traits::pointer_impls;

use crate::auxillary::Identifiable;


/***** AUXILLARY *****/
/// Convenience wrapper around [`Set`]s for when they are [infallible](std::convert::Infallible).
pub trait InfallibleSet<E>: Set<E, Error = Infallible> {
    /// Retrieves an element with a particular ID from this set.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A read-only reference to the relevant [`Set::Elem`] if an element with `id` existed,
    /// or else [`None`].
    fn get(&self, id: &<E as Identifiable>::Id) -> Option<&E>
    where
        E: Identifiable;

    /// Returns an iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields read-only references to every element.
    fn iter<'s>(&'s self) -> impl Iterator<Item = &'s E>
    where
        E: 's + Identifiable;
}

// Default impls
impl<E, T: Set<E, Error = Infallible>> InfallibleSet<E> for T {
    #[inline]
    fn get(&self, id: &<E as Identifiable>::Id) -> Option<&E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set<E>>::get(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn iter<'s>(&'s self) -> impl Iterator<Item = &'s E>
    where
        E: 's + Identifiable,
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
    /// If an element with the `elem`s ID already existed, it is removed from the set and
    /// returned. Otherwise, if it was new, [`None`] is returned.
    fn insert(&mut self, elem: E) -> Option<E>
    where
        E: Identifiable;



    /// Retrieves an element with a particular ID from this set such that it is mutable.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A mutable reference to the relevant [`Set::Elem`] if an element with `id` existed,
    /// or else [`None`].
    fn get_mut(&mut self, id: &<E as Identifiable>::Id) -> Option<&mut E>
    where
        E: Identifiable;

    /// Removes an element with a particular ID from this set and returns it.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// The relevant [`Set::Elem`] if an element with `id` existed, or else
    /// [`None`].
    fn remove(&mut self, id: &<E as Identifiable>::Id) -> Option<E>
    where
        E: Identifiable;

    /// Returns a mutable iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields mutable references to every element.
    fn iter_mut<'s>(&'s mut self) -> impl Iterator<Item = &'s mut E>
    where
        E: 's + Identifiable;
}

// Default impls
impl<E, T: Set<E, Error = Infallible> + SetMut<E>> InfallibleSetMut<E> for T {
    #[inline]
    fn insert(&mut self, elem: E) -> Option<E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut<E>>::insert(self, elem).unwrap_unchecked() }
    }

    #[inline]
    fn get_mut(&mut self, id: &<E as Identifiable>::Id) -> Option<&mut E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut<E>>::get_mut(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn remove(&mut self, id: &<E as Identifiable>::Id) -> Option<E>
    where
        E: Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut<E>>::remove(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn iter_mut<'s>(&'s mut self) -> impl Iterator<Item = &'s mut E>
    where
        E: 's + Identifiable,
    {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut<E>>::iter_mut(self).unwrap_unchecked() }
    }
}





/***** LIBRARY *****/
/// Defines read-only access to sets.
///
/// This is always how agents can access sets. However, they can only
/// [interface with a set mutably](SetMut) if that set is asynchronous.
#[pointer_impls]
pub trait Set<E> {
    /// The errors potentially thrown when interacting with the set.
    type Error: Error;


    /// Retrieves an element with a particular ID from this set.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A read-only reference to the relevant [`Set::Elem`] if an element with `id` existed,
    /// or else [`None`].
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn get(&self, id: &<E as Identifiable>::Id) -> Result<Option<&E>, Self::Error>
    where
        E: Identifiable;

    /// Returns an iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields read-only references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter<'s>(&'s self) -> Result<impl Iterator<Item = &'s E>, Self::Error>
    where
        E: 's + Identifiable;
}

// Default impls for std types.
impl<T: Identifiable> Set<T> for Vec<T> {
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
impl<T: Identifiable> Set<T> for HashMap<<T::Id as ToOwned>::Owned, T>
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



/// Defines mutable access to sets.
///
/// Agents can always [access sets immutably](Set). However, they can only do so mutably if that
/// set is asynchronous.
#[pointer_impls]
pub trait SetMut<E>: Set<E> {
    /// Inserts a new element into the set.
    ///
    /// # Arguments
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Returns
    /// If an element with the `elem`s ID already existed, it is removed from the set and
    /// returned. Otherwise, if it was new, [`None`] is returned.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn insert(&mut self, elem: E) -> Result<Option<E>, Self::Error>
    where
        E: Identifiable;



    /// Retrieves an element with a particular ID from this set such that it is mutable.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A mutable reference to the relevant [`Set::Elem`] if an element with `id` existed,
    /// or else [`None`].
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn get_mut(&mut self, id: &<E as Identifiable>::Id) -> Result<Option<&mut E>, Self::Error>
    where
        E: Identifiable;

    /// Removes an element with a particular ID from this set and returns it.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// The relevant [`Set::Elem`] if an element with `id` existed, or else
    /// [`None`].
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn remove(&mut self, id: &<E as Identifiable>::Id) -> Result<Option<E>, Self::Error>
    where
        E: Identifiable;

    /// Returns a mutable iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields mutable references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter_mut<'s>(&'s mut self) -> Result<impl Iterator<Item = &'s mut E>, Self::Error>
    where
        E: 's + Identifiable;
}

// Default impls for std types.
impl<T: Identifiable> SetMut<T> for Vec<T> {
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
impl<T> SetMut<T> for HashMap<<T::Id as ToOwned>::Owned, T>
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
