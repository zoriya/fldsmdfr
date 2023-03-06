use std::{env, path::Path};

use tokio::{net::UnixListener, io::{Interest, AsyncReadExt}};

use crate::manager::NotifyManager;

impl NotifyManager {
	pub async fn listen(&self) -> Result<(), zbus::Error> {
		let socket = Path::new("/tmp/fldsmdfr/socket.sock");
		let listener = UnixListener::bind(socket)?;
		loop {
			match listener.accept().await {
				Ok((stream, _addr)) => {
					tokio::spawn(async move {
						let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;

						if ready.is_readable() {
							let cmd = stream.read_to_string(dst)
						}
					});
				}
				Err(e) => {
					eprintln!("Could not accept connection: {}", e);
				}
			}
		}
	}
}

