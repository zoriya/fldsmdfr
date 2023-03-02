use zbus::zvariant::{DeserializeDict, SerializeDict, Type};
use serde::Serialize;

#[derive(Serialize)]
pub struct Notification {
	pub app_name: String,
	pub app_icon: String,
	pub summary: String,
	pub body: String,
	pub actions: Vec<String>,
	pub hints: Hints,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "dict")]
pub struct Hints {
	category: Option<String>,
	urgency: Option<u8>,
}
