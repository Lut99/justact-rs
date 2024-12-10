//  RUNTIME.rs
//    by Lut99
//
//  Created:
//    10 Dec 2024, 17:11:17
//  Last edited:
//    10 Dec 2024, 17:27:28
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the toplevel interface of a [`Runtime`]. While this is not
//!   in the ontology, it does hint at how it is supposed to be used.
//


/***** AUXILLARY *****/
/// Defines the view that agents or synchronizers have on the runtime.
#[derive(Clone, Copy, Debug)]
pub struct View<T, A, S, E> {
    /// The set of times that can possibly exist, including one current one.
    pub times:      T,
    /// The set of agreements that have been formulated.
    pub agreements: A,
    /// The set of messages that have been stated (and visible to this agent!).
    pub statements: S,
    /// The set of actions that have been enacted (and visible to this agent!).
    pub enacted:    E,
}





/***** LIBRARY *****/
/// Defines the toplevel [`Runtime`], which brings the ontology together.
pub trait Runtime {}
