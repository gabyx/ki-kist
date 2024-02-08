use clap::{Command, FromArgMatches as _, Parser, Subcommand as _};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kikist-cli")]
#[command(version = "0.0.1")]
#[command(about ="The client for kikist - a public/private key store.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

/// TODO: Try to collocate common fields with a macro, but its not possible inside struct fields.
#[derive(Parser, Debug)]
enum Subcommands {
    Generate,

    Store {
        #[arg(long, value_name = "HOST_URL")]
        host: String,

        #[arg(short, long)]
        access_token: Option<String>,

        #[arg(short, long)]
        password_file: Option<String>,
    },

    Get {
        #[arg(long)]
        host: String,

        #[arg(short, long)]
        access_token: Option<String>,

        #[arg(short, long)]
        password_file: Option<String>,
    },

    Sign {
        #[arg(short, long)]
        file: PathBuf,

        #[arg(long, value_name = "HOST_URL")]
        host: String,

        #[arg(short, long)]
        access_token: Option<String>,

        #[arg(short, long)]
        password_file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Subcommands::Generate => (),
        Subcommands::Store { .. } => (),
        Subcommands::Get { .. } => (),
        Subcommands::Sign { .. } => (),
    }
}
