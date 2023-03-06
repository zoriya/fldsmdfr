use std::time::Duration;

use tokio::{
	select,
	time::{sleep, Sleep},
};
use zbus::{dbus_proxy, export::ordered_stream::OrderedStreamExt, Connection, Result};

use crate::{notification::Notification, Format};

#[dbus_proxy(
	default_service = "org.freedesktop.Notifications",
	interface = "org.freedesktop.Notifications",
	default_path = "/org/freedesktop/Notifications"
)]
trait Manager {
	fn list(&self) -> Result<Vec<Notification>>;

	#[dbus_proxy(signal)]
	fn new_notification(&self, notif: Notification) -> fdo::Result<Notification>;
}

pub async fn listen(format: Format, clear: bool) -> Result<()> {
	let connection = Connection::session().await?;
	let manager = ManagerProxy::new(&connection).await?;
	let mut stream = manager.receive_new_notification().await?;
	let mut clearer: Sleep;
	let mut clear_running = false;

	loop {
		clearer = sleep(Duration::from_secs(8));
		select! {
			Some(notif) = stream.next() => {
				let notif = notif.args()?.notif;
				match format {
					Format::Json => println!("{}", serde_json::to_string(&notif).unwrap()),
					Format::Short => println!("{}: {}", notif.summary, notif.body),
				};
				if clear {
					clear_running = true;
				}
			},
			() = clearer, if clear_running => {
				match format {
					Format::Short => println!(""),
					Format::Json => println!("{}", serde_json::to_string(&Notification::default()).unwrap()),
				}
				clear_running = false;
			},
			else => { break }
		}
	}
	Ok(())
}

// TODO: strip \n on the short format.
// TODO: html unescape dbus messages.
pub async fn list(format: Format) -> Result<()> {
	let connection = Connection::session().await?;
	let proxy = ManagerProxy::new(&connection).await?;
	let pendings = proxy.list().await?;
	match format {
		Format::Json => println!("{}", serde_json::to_string(&pendings).unwrap()),
		Format::Short => {
			for notif in &pendings {
				println!("{}: {}", notif.summary, notif.body);
			}
		}
	}
	Ok(())
}
