//  MOD.rs
//    by Lut99
//
//  Created:
//    13 Jan 2025, 16:22:05
//  Last edited:
//    13 Jan 2025, 16:57:35
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
pub enum Selector<'i, I: ?Sized> {
    /// Send it to a specific agent.
    Agent(&'i I),
    /// Send it to all agents.
    All,
}
