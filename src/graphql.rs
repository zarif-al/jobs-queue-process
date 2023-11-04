/*
This module will contain graphql related code such as
- Structs needed to create the schema
    - Query Struct and its methods
    - Mutation struct and its methods
- Helper structs to define return types
*/
mod helper_structs;
mod mutation_root;
mod query_root;

pub use mutation_root::GraphQLMutationRoot;
pub use query_root::GraphQLQueryRoot;
