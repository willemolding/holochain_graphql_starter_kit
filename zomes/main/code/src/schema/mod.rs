use std::rc::Rc;
use std::cell::RefCell;

use juniper::{FieldResult, ID};

use crate::cache::Cache;

mod widget;
use widget::Widget;

/*=====================================
=            Input Objects            =
=====================================*/
// This is the process of defining a graphQL Input type
// https://graphql.org/learn/schema/#input-types

#[derive(GraphQLInputObject)]
#[graphql(description="An input object for example purposes only")]
struct ExampleInput {
    /// this rustdoc string (triple slash) will be visible when querying the endpoint for documentation
    some_required_field: i32,
    /// this field is optional
    some_optional_field: Option<String>,
}

/*=====  End of Input Objects  ======*/


/*
 * The query object is the top level object for the graphQL provider
 * Each field is something that can be queried. These take parameters to filter as needed
 */

pub struct Query;
graphql_object!(Query: Context |&self| {

    /// returns the API version. Mostly for testing/example purposes
    field apiVersion() -> FieldResult<&str> {
        Ok("1.0")
    }

    /// retrieve a single widget by its ID. Subqueries can then return info about this widget
    field widget(address: ID) -> FieldResult<Widget> {
        Ok(Widget{address})
    }

    /// return all the available widgets
    field widgets(&executor) -> FieldResult<Vec<Widget>> {
        Ok(Vec::new())
    }

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
// Without this cache there would be a get call for every field of every entry!
// No changes need to be made to the context for normal usage
pub struct Context {
    pub cache: Rc<RefCell<Cache>>
}
impl juniper::Context for Context {}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;
