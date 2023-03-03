use crate::notification::Hints;
use crate::notification::Notification;
use crate::notification::ServerInformation;
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
				id: self.next_id,
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

	fn get_server_information(&self) -> ServerInformation {
		ServerInformation {
			name: env!("CARGO_PKG_NAME").to_string(),
			vendor: "zoriya".to_string(),
			version: env!("CARGO_PKG_VERSION").to_string(),
			spec_version: "1.2".to_string(),
		}
	}

	fn list(&self) -> Vec<&Notification> {
		let mut ret: Vec<&Notification> = self.pendings.values().collect();
		ret.sort_by(|a, b| a.id.cmp(&b.id));
		ret
	}
}
