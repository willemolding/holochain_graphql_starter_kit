use std::fmt;

use juniper::ID;
use hdk::holochain_core_types::cas::content::Address;

#[derive(Clone)]
pub struct HID(ID);

impl fmt::Display for HID {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.write_str(&self.0.to_string())?;
		Ok(())
	}
}

// conversions to and from juniper HID
impl From<ID> for HID {
    fn from(jid: ID) -> Self {
        HID(jid)
    }
}

impl Into<ID> for HID {
    fn into(self) -> ID {
        self.0
    }
}


// conversions to and from holochain Address
impl From<Address> for HID {
    fn from(a: Address) -> Self {
        HID(a.to_string().into())
    }
}

impl Into<Address> for HID {
    fn into(self) -> Address {
        self.0.to_string().into()
    }
}
