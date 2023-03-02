use std::error::Error;
use zbus::{Connection, zvariant::{DeserializeDict, SerializeDict, Type}};
use zbus::dbus_interface;

struct NotifManager {
	next_id: u32,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "dict")]
struct Hints {
	category: Option<String>,
	urgency: Option<u8>,
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl NotifManager {
	fn notify(
		&mut self,
		app_name: &str,
		replaces_id: u32,
		app_icon: &str,
		summary: &str,
		body: &str,
		actions: Vec<&str>,
		hints: Hints,
		expire_timeout: i32,
	) -> u32 {
		self.next_id += 1;
		println!("{}: {}", summary, body);
		self.next_id
	}

	fn get_capabilities(&self) -> Vec<&str> {
		vec!["body", "actions", "body-images", "persistence", "icon-static"]
	}

	fn get_server_information(&self) -> [&str; 4] {
		[
			env!("CARGO_PKG_NAME"),
			"zoriya",
			env!("CARGO_PKG_VERSION"),
			"1.2",
		]
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let connection = Connection::session().await?;
	let manager = NotifManager { next_id: 0 };

	connection
		.object_server()
		.at("/org/freedesktop/Notifications", manager)
		.await?;
	connection
		.request_name("org.freedesktop.Notifications")
		.await?;

	loop {}
}
