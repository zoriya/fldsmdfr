use std::error::Error;
use zbus::{dbus_proxy, Connection};

#[dbus_proxy(
	default_service = "org.freedesktop.Notifications",
	interface = "org.freedesktop.Notifications",
	default_path = "/org/freedesktop/Notifications"
)]
trait Manager {
	fn list(&self, short: bool) -> zbus::Result<u32>;
}

pub async fn list(short: bool) -> Result<(), Box<dyn Error>> {
	let connection = Connection::session().await?;
	let proxy = ManagerProxy::new(&connection).await?;
	println!("{}", proxy.list(short).await?);
	Ok(())
}
