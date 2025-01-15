//  MOD.rs
//    by Lut99
//
//  Created:
//    13 Jan 2025, 16:22:05
//  Last edited:
//    15 Jan 2025, 17:41:43
//  Auto updated?
//    Yes
//
//  Description:
//!   Abstractly defines how sets (and maps) are implemented.
//

// Declare the modules
pub mod map;
pub mod set;


/***** LIBRARY *****/
/// Defines ways for agents to choose who to send updates to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Selector<I> {
    /// Send it to a specific agent.
    Agent(I),
    /// Send it to all agents.
    All,
}
impl<I> Selector<I> {
    /// Maps the identifier of the agent, if any.
    ///
    /// # Arguments
    /// - `callback`: Some [`FnOnce`] that will translate `I` to something else. Note it is only
    ///   called if this is a [`Selector::Agent`].
    ///
    /// # Returns
    /// A new Selector with the mapped identifier of an agent.
    #[inline]
    pub fn map<T>(self, callback: impl FnOnce(I) -> T) -> Selector<T> {
        match self {
            Self::Agent(id) => Selector::Agent(callback(id)),
            Self::All => Selector::All,
        }
    }
}
