//  ACTIONS.rs
//    by Lut99
//
//  Created:
//    11 Dec 2024, 10:06:41
//  Last edited:
//    16 Dec 2024, 15:16:32
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines actions, which enact certain effects.
//

use auto_traits::pointer_impls;

use crate::agreements::Agreement;
use crate::auxillary::{Actored, Identifiable, Timed};
use crate::messages::MessageSet;


/***** LIBRARY *****/
/// Defines an action that an agent can take.
///
/// Like [`Message`]s, actions are abstract because runtimes may wants to decide how they structure
/// the memory of the Action. In particular, they might want to collide the ID and the author.
#[pointer_impls]
pub trait Action: Actored + Identifiable + Timed {
    /// The type of messages this action uses.
    type Message: Identifiable;


    /// The agreement that forms the basis of the action.
    ///
    /// # Returns
    /// An [`Agreement`] to base the action on.
    fn basis(&self) -> &Agreement<Self::Message, Self::Timestamp>;

    /// The justification that should satisfy the agreement.
    ///
    /// Note that this should include the statement embedded by the agreement as well.
    ///
    /// # Returns
    /// A [`MessageSet`] encoding the statements in the justification.
    fn justification(&self) -> &MessageSet<Self::Message>;
}
