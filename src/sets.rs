//  SETS.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 10:40:27
//  Last edited:
//    10 Dec 2024, 14:03:47
//  Auto updated?
//    Yes
//
//  Description:
//!   Abstractly defines how a set is implemented.
//

use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;

use crate::auxillary::Identifiable;


/***** AUXILLARY *****/
/// Convenience wrapper around [`Set`]s for when they are [infallible](std::convert::Infallible).
pub trait InfallibleSet: Set<Error = Infallible> {
    /// Retrieves an element with a particular ID from this set.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A read-only reference to the relevant [`Set::Elem`] if an element with `id` existed,
    /// or else [`None`].
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Option<&Self::Elem>;

    /// Returns an iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields read-only references to every element.
    fn iter(&self) -> impl Iterator<Item = &Self::Elem>;
}

// Default impls
impl<T: Set<Error = Infallible>> InfallibleSet for T {
    #[inline]
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Option<&Self::Elem> {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set>::get(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = &Self::Elem> {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as Set>::iter(self).unwrap_unchecked() }
    }
}



/// Convenience wrapper around [`SetMut`]s for when they are
/// [infallible](std::convert::Infallible).
pub trait InfallibleSetMut: Set<Error = Infallible> + SetMut {
    /// Inserts a new element into the set.
    ///
    /// # Arguments
    /// - `elem`: The [`Set::Elem`] to add to the set.
    ///
    /// # Returns
    /// If an element with the `elem`s ID already existed, it is removed from the set and
    /// returned. Otherwise, if it was new, [`None`] is returned.
    fn insert(&mut self, elem: Self::Elem) -> Option<Self::Elem>;



    /// Retrieves an element with a particular ID from this set such that it is mutable.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// A mutable reference to the relevant [`Set::Elem`] if an element with `id` existed,
    /// or else [`None`].
    fn get_mut(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Option<&mut Self::Elem>;

    /// Removes an element with a particular ID from this set and returns it.
    ///
    /// # Arguments
    /// - `id`: The identifier of the element to retrieve.
    ///
    /// # Returns
    /// The relevant [`Set::Elem`] if an element with `id` existed, or else
    /// [`None`].
    fn remove(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Option<Self::Elem>;

    /// Returns a mutable iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields mutable references to every element.
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Elem>;
}

// Default impls
impl<T: Set<Error = Infallible> + SetMut> InfallibleSetMut for T {
    #[inline]
    fn insert(&mut self, elem: Self::Elem) -> Option<Self::Elem> {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut>::insert(self, elem).unwrap_unchecked() }
    }

    #[inline]
    fn get_mut(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Option<&mut Self::Elem> {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut>::get_mut(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn remove(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Option<Self::Elem> {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut>::remove(self, id).unwrap_unchecked() }
    }

    #[inline]
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Elem> {
        // SAFETY: It is physically impossible for users to express `Err(...)` due to the inability
        // to construct `Infallible`
        unsafe { <T as SetMut>::iter_mut(self).unwrap_unchecked() }
    }
}





/***** LIBRARY *****/
/// Defines read-only access to sets.
///
/// This is always how agents can access sets. However, they can only
/// [interface with a set mutably](SetMut) if that set is asynchronous.
pub trait Set {
    /// The element contained within.
    type Elem: Identifiable;
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
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&Self::Elem>, Self::Error>;

    /// Returns an iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields read-only references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter(&self) -> Result<impl Iterator<Item = &Self::Elem>, Self::Error>;
}

// Default impls for std types.
impl<T: Identifiable> Set for Vec<T> {
    type Elem = T;
    type Error = Infallible;


    #[inline]
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&Self::Elem>, Self::Error> {
        for elem in self {
            if elem.id() == id {
                return Ok(Some(elem));
            }
        }
        Ok(None)
    }

    #[inline]
    fn iter(&self) -> Result<impl Iterator<Item = &Self::Elem>, Self::Error> { Ok(<[T]>::iter(self)) }
}
impl<T: Identifiable> Set for HashMap<T::Id, T> {
    type Elem = T;
    type Error = Infallible;


    #[inline]
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&Self::Elem>, Self::Error> { Ok(<Self>::get(self, id)) }

    #[inline]
    fn iter(&self) -> Result<impl Iterator<Item = &Self::Elem>, Self::Error> { Ok(<Self>::values(self)) }
}

// Default impls for pointer-like types.
impl<'a, T: Set> Set for &'a T {
    type Elem = T::Elem;
    type Error = T::Error;

    #[inline]
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&Self::Elem>, Self::Error> { <T as Set>::get(self, id) }

    #[inline]
    fn iter(&self) -> Result<impl Iterator<Item = &Self::Elem>, Self::Error> { <T as Set>::iter(self) }
}
impl<'a, T: Set> Set for &'a mut T {
    type Elem = T::Elem;
    type Error = T::Error;

    #[inline]
    fn get(&self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&Self::Elem>, Self::Error> { <T as Set>::get(self, id) }

    #[inline]
    fn iter(&self) -> Result<impl Iterator<Item = &Self::Elem>, Self::Error> { <T as Set>::iter(self) }
}



/// Defines mutable access to sets.
///
/// Agents can always [access sets immutably](Set). However, they can only do so mutably if that
/// set is asynchronous.
pub trait SetMut: Set {
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
    fn insert(&mut self, elem: Self::Elem) -> Result<Option<Self::Elem>, Self::Error>;



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
    fn get_mut(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&mut Self::Elem>, Self::Error>;

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
    fn remove(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<Self::Elem>, Self::Error>;

    /// Returns a mutable iterator over the elements in this set.
    ///
    /// # Returns
    /// An [`Iterator`] over [`Set::Elem`] that yields mutable references to every element.
    ///
    /// # Errors
    /// When this function errors is completely implementation-dependent.
    fn iter_mut(&mut self) -> Result<impl Iterator<Item = &mut Self::Elem>, Self::Error>;
}

// Default impls for std types.
impl<T: Identifiable> SetMut for Vec<T> {
    #[inline]
    fn insert(&mut self, mut new_elem: Self::Elem) -> Result<Option<Self::Elem>, Self::Error> {
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
    fn get_mut(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&mut Self::Elem>, Self::Error> {
        for elem in self {
            if elem.id() == id {
                return Ok(Some(elem));
            }
        }
        Ok(None)
    }

    #[inline]
    fn remove(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<Self::Elem>, Self::Error> {
        for (i, elem) in <[T]>::iter_mut(self).enumerate() {
            if elem.id() == id {
                return Ok(Some(self.swap_remove(i)));
            }
        }
        Ok(None)
    }

    #[inline]
    fn iter_mut(&mut self) -> Result<impl Iterator<Item = &mut Self::Elem>, Self::Error> { Ok(<[T]>::iter_mut(self)) }
}
impl<T> SetMut for HashMap<T::Id, T>
where
    T: Identifiable,
    T::Id: Clone,
{
    #[inline]
    fn insert(&mut self, elem: Self::Elem) -> Result<Option<Self::Elem>, Self::Error> { Ok(<Self>::insert(self, elem.id().clone(), elem)) }

    #[inline]
    fn get_mut(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&mut Self::Elem>, Self::Error> { Ok(<Self>::get_mut(self, id)) }

    #[inline]
    fn remove(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<Self::Elem>, Self::Error> { Ok(<Self>::remove(self, id)) }

    #[inline]
    fn iter_mut(&mut self) -> Result<impl Iterator<Item = &mut Self::Elem>, Self::Error> { Ok(<Self>::values_mut(self)) }
}

// Default impls for pointer-like types.
impl<'a, T: SetMut> SetMut for &'a mut T {
    #[inline]
    fn insert(&mut self, elem: Self::Elem) -> Result<Option<Self::Elem>, Self::Error> { <T as SetMut>::insert(self, elem) }

    #[inline]
    fn get_mut(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<&mut Self::Elem>, Self::Error> { <T as SetMut>::get_mut(self, id) }

    #[inline]
    fn remove(&mut self, id: &<Self::Elem as Identifiable>::Id) -> Result<Option<Self::Elem>, Self::Error> { <T as SetMut>::remove(self, id) }

    #[inline]
    fn iter_mut(&mut self) -> Result<impl Iterator<Item = &mut Self::Elem>, Self::Error> { <T as SetMut>::iter_mut(self) }
}
