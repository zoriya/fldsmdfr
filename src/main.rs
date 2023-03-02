use std::{error::Error, future::pending};
use zbus::{dbus_interface, ConnectionBuilder};

struct NotifManager {
	next_id: u32,
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
		// actions: as,
		// hints: a{sv},
		expire_timeout: i32,
	) -> u32 {
		self.next_id += 1;
		println!("{}: {}", summary, body);
		self.next_id
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let manager = NotifManager { next_id: 0 };
	let _ = ConnectionBuilder::session()?
		.name("org.freedesktop.Notifications")?
		.serve_at("/org/freedesktop/Notifications", manager)?
		.build()
		.await?;

	pending::<()>().await;
	Ok(())
}
