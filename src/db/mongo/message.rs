/*
This module contains the code to define and work with the
'Message' entity.
*/
mod definition;
mod get_messages;
mod insert;

pub use definition::DBMessage;
pub use get_messages::get_messages;
pub use insert::insert;
