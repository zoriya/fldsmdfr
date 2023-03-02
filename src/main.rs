mod daemon;
mod notification;
mod client;

use clap::{Parser, Subcommand};
use daemon::NotifyManager;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command {
	/// Run the notification daemon.
	Daemon,

	/// Listen to new notifications.
	Listen {
		/// Use json instead of plain-text
		#[arg(short, long)]
		json: bool,

		/// Print an empty line/json object when the notification should clear.
		#[arg(short, long)]
		clear: bool,
	},

	/// List pending notifications
	List {
		/// Use a quick overview instead of json, one notification per line
		#[arg(short, long)]
		short: bool,
	},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let cli = Args::parse();

	match cli.command {
		Command::Daemon => NotifyManager::new().start().await,
		Command::List { short } => client::list(short).await,
		Command::Listen { .. } => unimplemented!(),
	}
}
