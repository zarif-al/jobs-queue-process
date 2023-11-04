/*
This module will contain database related code such as
    - getting database connections
    - carrying our CRUD operations against databases
*/
mod common;
mod mongo;
mod redis;

pub use mongo::connect::mongo_conn;
pub use mongo::message as mongo_message;
pub use mongo::message::get_messages as get_mongo_messages;
pub use redis::connect::redis_conn;
