mod client;
mod daemon;
mod notification;

use clap::{Parser, Subcommand, ValueEnum};
use daemon::NotifyManager;
use zbus::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
// TODO: Fix this replace that does not work.
#[command(replace("-j", ["--format json"]))]
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
		/// Select the format to display notifications.
		#[arg(short, long, default_value = "short")]
		format: Format,

		/// Print an empty line/json object when the notification should clear.
		#[arg(short, long)]
		clear: bool,
	},

	/// List pending notifications
	List {
		/// Select the format to display notifications.
		#[arg(short, long, default_value = "short")]
		format: Format,
	},
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
	/// A short, human-readable display.
	Short,
	/// A json object with every informations available
	Json,
}

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Args::parse();

	match cli.command {
		Command::Daemon => NotifyManager::new().start().await,
		Command::List { format } => client::list(format).await,
		Command::Listen { format, clear } => client::listen(format, clear).await,
	}
}
