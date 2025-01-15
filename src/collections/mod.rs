//  MOD.rs
//    by Lut99
//
//  Created:
//    13 Jan 2025, 16:22:05
//  Last edited:
//    15 Jan 2025, 17:31:51
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
pub enum Selector<'i, I: ?Sized> {
    /// Send it to a specific agent.
    Agent(&'i I),
    /// Send it to all agents.
    All,
}
