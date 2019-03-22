use std::rc::Rc;
use std::cell::RefCell;

use juniper::{FieldResult, ID};
use crate::cache::Cache;

mod widget;
use widget::Widget;
use super::widget_entry;

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
    description: "The top level query object for the holochain+graphQL starter kit!"

    /// returns the API version. Mostly for testing/example purposes
    field apiVersion() -> FieldResult<&str> {
        Ok("1.0")
    }

    /// retrieve a single widget by its ID. Subqueries can then return info about this widget
    field widget(address: ID) -> FieldResult<Widget> {
        Ok(Widget{address: address.into()})
    }

    /// create/return the root of the widget tree
    field rootWidget(&executor) -> FieldResult<Widget> {
        let root_widget_address = widget_entry::add_widget(
            executor.context().cache.borrow_mut(),
            "root".to_string()
        )?;
        Ok(Widget{address: root_widget_address.into()})
    }

});


/*
 * the mutation object is what allows the consumer to change the data stored in the store
 * In holochain the store is the DHT. You also need to be sure you allow some pattern (such as links)
 * such that the values can be retrieved again later
 */
pub struct Mutation;
graphql_object!(Mutation: Context |&self| {
    description: "by convention these callbacks are allowed to mutate the DHT or local chain"

    /// Add a new widget with a given description
    field addWidget(&executor, description: String) -> FieldResult<Widget> {
        let new_widget_address = widget_entry::add_widget(
            executor.context().cache.borrow_mut(),
            description
        )?;
        Ok(Widget{address: new_widget_address.into()})
    }

    /// Add one widget as a subwidget of another. Can be used to make widget trees!
    field appendSubwidget(&executor, parent_address: ID, child_address: ID) -> FieldResult<Widget> {
        widget_entry::add_subwidget(
            executor.context().cache.borrow_mut(),
            &parent_address.to_string().into(),
            &child_address.to_string().into(),
        )?;
        Ok(Widget{address: child_address.into()})
    }

    field addSubwidgetAsChild(&executor, parent_address: ID, description: String) -> FieldResult<Widget> {
        let new_widget_address = widget_entry::add_widget(
            executor.context().cache.borrow_mut(),
            description
        )?;
        widget_entry::add_subwidget(
            executor.context().cache.borrow_mut(),
            &parent_address.to_string().into(),
            &new_widget_address.to_string().into(),
        )?;
        Ok(Widget{address: new_widget_address.into()})
    }

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
