use serde::{Deserialize, Serialize, Serializer};
use zbus::zvariant::{self, DeserializeDict, SerializeDict, Type};

#[derive(Debug, Deserialize, Serialize, Type, Default, Clone)]
#[zvariant(signature = "dict")]
pub struct Notification {
	pub id: u32,
	pub app_name: String,
	pub app_icon: String,
	pub summary: String,
	pub body: String,
	pub actions: Vec<String>,
	pub expire_timeout: Option<u32>,
	pub urgency: Option<u8>,
	pub category: Option<String>,
}

impl<'a> From<zvariant::Value<'a>> for Notification {
	fn from(value: zvariant::Value<'a>) -> Self {
		println!("{:?}", value);
		todo!()
	}
}

#[derive(Debug, DeserializeDict, SerializeDict, Type, Default)]
#[zvariant(signature = "dict")]
pub struct Hints {
	pub category: Option<String>,
	pub urgency: Option<u8>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
pub struct ServerInformation {
	/// The product name of the server.
	pub name: String,

	/// The vendor name. For example "KDE," "GNOME," "freedesktop.org" or "Microsoft".
	pub vendor: String,

	/// The server's version number.
	pub version: String,

	/// The specification version the server is compliant with.
	pub spec_version: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[repr(i32)]
pub enum CloseReason {
	Expired,
	Dismissed,
	Manual,
	Undefined,
}
