use crate::dbox::DBox;
use crate::notification::{CloseReason, Hints, Notification, ServerInformation};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use zbus::{dbus_interface, fdo, Connection, Result, SignalContext};

pub struct NotifyManager {
	next_id: u32,
	pendings: Arc<Mutex<HashMap<u32, Notification>>>,
}

impl NotifyManager {
	pub fn new() -> NotifyManager {
		NotifyManager {
			next_id: 1,
			pendings: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	pub async fn start(self) -> Result<()> {
		let connection = Connection::session().await?;
		let pendings = self.pendings.clone();

		connection
			.object_server()
			.at("/org/freedesktop/Notifications", self)
			.await?;
		connection
			.request_name("org.freedesktop.Notifications")
			.await?;

		let ctxt = SignalContext::new(&connection, "/org/freedesktop/Notifications")?;
		loop {
			sleep(Duration::from_secs(1)).await;
			let mut pend = pendings.lock().unwrap();
			let mut to_remove: Vec<u32> = Vec::new();

			for notif in pend.values_mut() {
				notif.expire_timeout = notif.expire_timeout.map(|x| x - (1000));
				if notif.expire_timeout < Some(1000) {
					to_remove.push(notif.id);
				}
			}
			for id in to_remove {
				pend.remove(&id);
				Self::notification_closed(&ctxt, id, CloseReason::Expired).await?
			}
		}
	}
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl NotifyManager {
	async fn notify(
		&mut self,
		#[zbus(signal_context)] ctxt: SignalContext<'_>,
		app_name: String,
		replaces_id: u32,
		app_icon: String,
		summary: String,
		body: String,
		actions: Vec<String>,
		hints: Hints,
		expire_timeout: i32,
	) -> fdo::Result<u32> {
		println!("{}: {}", summary, body);
		let notif = Notification {
			id: self.next_id,
			app_name,
			app_icon,
			summary,
			body,
			actions,
			urgency: hints.urgency,
			category: hints.category,
			expire_timeout: match expire_timeout {
				0 => None,
				x if x < 0 => Some(8000),
				x => Some(x as u32),
			},
		};
		// Self::new_notification(&ctxt, DBox::new(&notif)).await?;
		self.pendings.lock().unwrap().insert(self.next_id, notif);
		if replaces_id == 0 {
			self.next_id += 1;
			Ok(self.next_id)
		} else {
			Ok(replaces_id)
		}
	}

	// async fn close_notification(
	// 	&mut self,
	// 	#[zbus(signal_context)] ctxt: SignalContext<'_>,
	// 	id: u32,
	// ) -> fdo::Result<()> {
	// 	self.pendings
	// 		.lock()
	// 		.unwrap()
	// 		.remove(&id)
	// 		.ok_or(zbus::Error::Failure(String::from("Value already removed")))?;
	// 	Self::notification_closed(&ctxt, id, CloseReason::Manual)
	// 		.await
	// 		.map_err(|x| zbus::fdo::Error::from(x))
	// }

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

	#[dbus_interface(signal)]
	async fn notification_closed(
		ctxt: &SignalContext<'_>,
		id: u32,
		reason: CloseReason,
	) -> Result<()>;
}
