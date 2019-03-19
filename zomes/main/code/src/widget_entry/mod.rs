use core::cell::RefMut;
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address,
        dna::entry_types::Sharing,
        error::HolochainError,
        json::JsonString,
        entry::Entry,
    },
};
use crate::cache::{self, Cache};

const SUBWIDGET_LINK_TAG: &str = "subwidgets";

/**
 * This struct is what holochain uses for storing and validating a widget
 */
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct WidgetEntry {
    pub description: String
}

/**
 * This function is used in define_zome! to register the widget entry
 * Defines the validation and links for the entry
 */
pub fn def() -> ValidatingEntryType {
    entry!(
        name: "widget",
        description: "Just your average widget",
        sharing: Sharing::Public,
        native_type: WidgetEntry,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_message: WidgetEntry, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
        	to!( // widgets can have subwidgets allowing infinite widget trees!
                "widget",
                tag: SUBWIDGET_LINK_TAG,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub fn get_widget(cache: RefMut<Cache>, widget_address: &Address) -> ZomeApiResult<WidgetEntry> {
	cache::get_as_type(cache, widget_address)
}

pub fn get_subwidgets(cache: RefMut<Cache>, base_widget_address: &Address) -> ZomeApiResult<Vec<Address>> {
	Ok(cache.get_links(base_widget_address, SUBWIDGET_LINK_TAG)?
		.addresses()
		.to_owned())
}

pub fn add_widget(mut cache: RefMut<Cache>, description: String) -> ZomeApiResult<Address> {
	cache.commit_entry(&Entry::App(
		"widget".into(),
		WidgetEntry{description}.into()
	))
}

pub fn add_subwidget(mut cache: RefMut<Cache>, base_address: &Address, target_address: &Address) -> ZomeApiResult<()> {
	cache.link_entries(base_address, target_address, SUBWIDGET_LINK_TAG)
}
