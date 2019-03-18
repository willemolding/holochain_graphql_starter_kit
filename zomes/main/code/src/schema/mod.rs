use std::rc::Rc;
use std::cell::RefCell;

use juniper::{FieldError, FieldResult, Value, ID};

use crate::cache::Cache;

/*=====================================
=            Input Objects            =
=====================================*/
// This is the process of defining a graphQL Input object that a function will receive
#[derive(GraphQLInputObject)]
struct ExampleInput {
    participant_ids: Option<Vec<Option<String>>>,
}

/*=====  End of Input Objects  ======*/


/*
 * The query object is the top level object for the graphQL provider
 * Each field is something that can be queried. These take parameters to filter as needed
 */

pub struct Query;
graphql_object!(Query: Context |&self| {
});


/*
 * the mutation object is what allows the consumer to change the data stored in the store
 * In holochain the store is the DHT. You also need to be sure you allow some pattern (such as links)
 * such that the values can be retrieved again later
 */
pub struct Mutation;
graphql_object!(Mutation: Context |&self| {
});




// define the context struct that is passed between all field calls in a query
// This is used to cache holochain calls to make queries much more efficient
pub struct Context {
    pub cache: Rc<RefCell<Cache>>
}
impl juniper::Context for Context {}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;
