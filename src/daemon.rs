use crate::notification::Hints;
use crate::notification::Notification;
use std::collections::HashMap;
use std::error::Error;
use zbus::dbus_interface;
use zbus::Connection;

pub struct NotifyManager {
	next_id: u32,
	pendings: HashMap<u32, Notification>,
}

impl NotifyManager {
	pub fn new() -> NotifyManager {
		NotifyManager {
			next_id: 1,
			pendings: HashMap::new(),
		}
	}

	pub async fn start(self) -> Result<(), Box<dyn Error>> {
		let connection = Connection::session().await?;

		connection
			.object_server()
			.at("/org/freedesktop/Notifications", self)
			.await?;
		connection
			.request_name("org.freedesktop.Notifications")
			.await?;

		loop {}
	}
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl NotifyManager {
	fn notify(
		&mut self,
		app_name: String,
		replaces_id: u32,
		app_icon: String,
		summary: String,
		body: String,
		actions: Vec<String>,
		hints: Hints,
		expire_timeout: i32,
	) -> u32 {
		println!("{}: {}", summary, body);
		self.pendings.insert(
			self.next_id,
			Notification {
				app_name,
				app_icon,
				summary,
				body,
				actions,
				hints,
			},
		);
		if replaces_id == 0 {
			self.next_id += 1;
			self.next_id
		} else {
			replaces_id
		}
	}

	fn get_capabilities(&self) -> Vec<&str> {
		vec![
			"body",
			"actions",
			"body-images",
			"persistence",
			"icon-static",
		]
	}

	fn get_server_information(&self) -> [&str; 4] {
		[
			env!("CARGO_PKG_NAME"),
			"zoriya",
			env!("CARGO_PKG_VERSION"),
			"1.2",
		]
	}

	fn list(&self, short: bool) -> String {
		if short {
			unreachable!()
			// self.pendings.values().map(|x| x.summary).join("\n")
		} else {
			serde_json::to_string(&self.pendings).unwrap()
		}
	}
}
