use zbus::{dbus_proxy, Connection, Result};

use crate::{notification::Notification, Format};

#[dbus_proxy(
	default_service = "org.freedesktop.Notifications",
	interface = "org.freedesktop.Notifications",
	default_path = "/org/freedesktop/Notifications"
)]
trait Manager {
	fn list(&self) -> Result<Vec<Notification>>;
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
