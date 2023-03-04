use serde::{Deserialize, Serialize};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Notification {
	pub id: u32,
	pub app_name: String,
	pub app_icon: String,
	pub summary: String,
	pub body: String,
	pub actions: Vec<String>,
	pub hints: Hints,
}

#[derive(Debug, DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "dict")]
pub struct Hints {
	category: Option<String>,
	urgency: Option<u8>,
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
