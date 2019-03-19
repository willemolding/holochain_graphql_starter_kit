use juniper::{FieldResult, ID};
use crate::Context;
use super::widget_entry;

/*
 * Each GraphQL object which is also a Holochain entry must be defined by a single struct with an address field
 * This address is the hash/DHT address in Holochain and is all that is needed to uniquely define an object
 * Do not include any of the entry fields here! They belong in the next macro
*/
#[derive(Clone)]
pub struct Widget {
    pub address: ID,
}

/*
 * This macro turns the struct into a fully fledged GraphQL object complete with fields which may take parameters
 * Every Holochain GraphQL object needs to expose an address field
*/
graphql_object!(Widget: Context |&self| {

	/// The holochain hash/address of the widget entry
	field address(&executor) -> FieldResult<&ID> {
		Ok(&self.address)
	}

	/// The description this widget holds. That is all it really does.
	field description(&executor) -> FieldResult<String> {
		let widget = widget_entry::get_widget(
			executor.context().cache.borrow_mut(),
			&self.address.to_string().into()
		)?;
		Ok(widget.description)
	}

	/// Retrieve all the subwidgets attached to this one as an array
	field subwidgets(&executor) -> FieldResult<Vec<Widget>> {
		let subwidget_addresses = widget_entry::get_subwidgets(
			executor.context().cache.borrow_mut(),
			&self.address.to_string().into()
		)?;

		let widgets = subwidget_addresses.iter().map(|address| {
			Widget{address: address.to_string().into()}
		}).collect();

		Ok(widgets)
	}

});
