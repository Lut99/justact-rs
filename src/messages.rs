//  MESSAGES.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 11:43:49
//  Last edited:
//    21 Jan 2025, 14:58:43
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines messages & message sets.
//

use std::collections::HashSet;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter, Result as FResult};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use std::sync::Arc;

use crate::auxillary::Authored;
use crate::collections::set::{Set, SetSync};


/***** HELPER MACROS *****/
/// Implements [`Message`] for pointer-like types.
macro_rules! message_ptr_impl {
    ('a, $ty:ty) => {
        impl<'a, T: Message> Message for $ty {
            type Payload = <T as Message>::Payload;

            #[inline]
            fn payload(&self) -> &Self::Payload { <T as Message>::payload(self) }
        }
    };

    ($ty:ty) => {
        impl<T: Message> Message for $ty {
            type Payload = <T as Message>::Payload;

            #[inline]
            fn payload(&self) -> &Self::Payload { <T as Message>::payload(self) }
        }
    };
}





/***** LIBRARY *****/
/// Defines a single message.
///
/// This is abstract, and not a concrete data structure, because runtimes may wants to decide how
/// they structure the memory of the Message. In particular, messages might be
/// [`Arc`](std::sync::Arc)'ed, and they might want to collide the ID and the author.
pub trait Message: Authored + Eq + Hash {
    /// Defines the type of content carried by this message.
    type Payload: ?Sized;

    /// Returns the payload of this message.
    ///
    /// # Returns
    /// An immutable reference to the internal [`Message::Payload`].
    fn payload(&self) -> &Self::Payload;


    /// Optional method that will yield a human-friendly identifier for this message.
    ///
    /// # Returns
    /// A string that identifies this message, friendly. If it's not implemented, it yields "???".
    #[inline]
    fn human_id(&self) -> &str { "???" }
}

// Pointer-like implementations
message_ptr_impl!('a, &'a T);
message_ptr_impl!('a, &'a mut T);
message_ptr_impl!(Box<T>);
message_ptr_impl!(Rc<T>);
message_ptr_impl!(Arc<T>);



/// Defines a constructor for a message.
///
/// This is a more powerful version of a message that can also be constructed, but needn't be one itself.
pub trait ConstructableMessage: Clone + Message
where
    Self::AuthorId: ToOwned,
    Self::Payload: ToOwned,
{
    /// Constructor for a new message with the given ID, author and payload.
    ///
    /// # Arguments
    /// - `author_id`: The identifier of the message's author.
    /// - `payload`: The payload to add to the message.
    ///
    /// # Returns
    /// A new Message.
    fn new(author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized;
}

// Manual pointer impls (for some of them)
impl<T> ConstructableMessage for Box<T>
where
    T: ConstructableMessage,
    T::AuthorId: ToOwned,
    T::Payload: ToOwned,
{
    #[inline]
    fn new(author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized,
    {
        Box::new(<T as ConstructableMessage>::new(author_id, payload))
    }
}
impl<T> ConstructableMessage for Rc<T>
where
    T: ConstructableMessage,
    T::AuthorId: ToOwned,
    T::Payload: ToOwned,
{
    #[inline]
    fn new(author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized,
    {
        Rc::new(<T as ConstructableMessage>::new(author_id, payload))
    }
}
impl<T> ConstructableMessage for Arc<T>
where
    T: ConstructableMessage,
    T::AuthorId: ToOwned,
    T::Payload: ToOwned,
{
    #[inline]
    fn new(author_id: <Self::AuthorId as ToOwned>::Owned, payload: <Self::Payload as ToOwned>::Owned) -> Self
    where
        Self: Sized,
    {
        Arc::new(<T as ConstructableMessage>::new(author_id, payload))
    }
}



/// Defines a bunch of messages.
#[derive(Clone, Debug)]
pub struct MessageSet<M> {
    /// The messages.
    data: HashSet<M>,
}

// Constructors
impl<M> Default for MessageSet<M> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl<M> MessageSet<M> {
    /// Constructor for the MessageSet that initializes it without elements.
    ///
    /// # Returns
    /// A new MessageSet, ready to store messages.
    #[inline]
    pub fn new() -> Self { Self { data: HashSet::new() } }

    /// Constructor for the MessageSet that initializes it without elements, but with the memory
    /// capacity for at least a specified number.
    ///
    /// This is useful for when you are expecting to put some things in there so that you only have
    /// to allocate once.
    ///
    /// # Arguments
    /// - `capacity`: The minimum number of elements that the new set should be able to store
    ///   before having to re-allocate. For optimization/alignment purposes, the actually reserved
    ///   capacity may be higher (see [`HashMap::with_capacity()`]).
    ///
    /// # Returns
    /// A new MessageSet, ready to store messages.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self { Self { data: HashSet::with_capacity(capacity) } }
}

// Ops
impl<M: Eq> Eq for MessageSet<M> {}
impl<M: Hash> Hash for MessageSet<M> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        /* We use the sort-by-hash-trick */
        // First, get a **deterministic** ordering on the items. This is important, as the
        // conceptually same set must always give the same ordering!
        let mut elems: Vec<&M> = self.data.iter().collect();
        elems.sort_by_key(|m| {
            // Now comes the real trick: we simply hash the element first and then sort the hashes.
            // NOTE: It is really important that the hasher used between `hash()`-calls of the set
            // as a whole produces the same hashes; so we won't use `RandomState` here. This does
            // make the sorting (but only the sorting!) vulnerable to HashDoS.
            let mut hasher = DefaultHasher::new();
            m.hash(&mut hasher);
            hasher.finish()
        });

        // Using this ordering, we can hash the array to get a predictable hash regardless of set
        // order.
        elems.hash(state)
    }
}
impl<M: PartialEq> PartialEq for MessageSet<M> {
    fn eq(&self, other: &Self) -> bool {
        /* We use the cross-out-opponents trick */
        // It's important that a difference in length is caught early, both for efficiency and for
        // correctness at the end.
        if self.data.len() != other.data.len() {
            return false;
        }

        // Let's get the list of items in `other`.
        // NOTE: We _don't_ use a hashset here as we don't want to assume `Hash`.
        let mut to_cross: Vec<&M> = self.data.iter().collect();

        // Then, iterate through our own elements and find them in `other`.
        for elem in &self.data {
            // NOTE: Our own `Set::iter()`-impl is in the way here, awkward xD
            if let Some(i) = <[_]>::iter(&to_cross).enumerate().find_map(|(i, &e)| if elem == e { Some(i) } else { None }) {
                // We found it, cross it out
                to_cross.swap_remove(i);
            } else {
                // We didn't find the element, it does not exist, they are not equal!
                return false;
            }
        }

        // If we got here, `self \subseteq other`. Further, `self \cap other = \emptyset`, since
        // `self` and `other` necessarily have the same length. Thus: `self == other`!
        true
    }
}

// Justact impls
impl<M: Eq + Hash> Set<M> for MessageSet<M> {
    type Error = Infallible;

    #[inline]
    fn get(&self, elem: &M) -> Result<Option<&M>, Self::Error> { Ok(self.data.get(elem)) }

    #[inline]
    fn iter<'s>(&'s self) -> Result<impl 's + Iterator<Item = &'s M>, Self::Error>
    where
        M: 's,
    {
        Ok(self.data.iter())
    }

    #[inline]
    fn len(&self) -> Result<usize, Self::Error> { Ok(self.data.len()) }
}
impl<M: Eq + Hash> SetSync<M> for MessageSet<M> {
    #[inline]
    fn add(&mut self, elem: M) -> Result<bool, Self::Error> { Ok(self.data.insert(elem)) }

    #[inline]
    fn clear(&mut self) -> Result<(), Self::Error> { Ok(self.data.clear()) }
}

// Serde
#[cfg(feature = "serde")]
impl<'de, M: Eq + Hash + serde::Deserialize<'de>> serde::Deserialize<'de> for MessageSet<M> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // https://serde.rs/deserialize-map.html
        struct Visitor<M> {
            _m: std::marker::PhantomData<M>,
        }
        impl<'de, M> serde::de::Visitor<'de> for Visitor<M>
        where
            M: Eq + Hash + serde::Deserialize<'de>,
        {
            type Value = MessageSet<M>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> FResult { write!(f, "a MessageSet (map of messages)") }

            #[inline]
            fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut set = MessageSet::with_capacity(access.size_hint().unwrap_or(0));
                while let Some(msg) = access.next_element::<M>()? {
                    set.add(msg).unwrap();
                }
                Ok(set)
            }
        }

        // Run the deserialize
        deserializer.deserialize_seq(Visitor { _m: std::marker::PhantomData::<M> })
    }
}
#[cfg(feature = "serde")]
impl<M: serde::Serialize> serde::Serialize for MessageSet<M> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq as _;
        let mut access = serializer.serialize_seq(Some(self.data.len()))?;
        for msg in self.data.iter() {
            access.serialize_element(msg)?;
        }
        access.end()
    }
}

// From
impl<I: IntoIterator<Item = M>, M: Eq + Hash> From<I> for MessageSet<M> {
    #[inline]
    fn from(value: I) -> Self { MessageSet { data: value.into_iter().collect() } }
}
impl<M: Eq + Hash> FromIterator<M> for MessageSet<M> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = M>>(iter: T) -> Self { MessageSet { data: iter.into_iter().map(|m| (m.id().to_owned(), m)).collect() } }
}
