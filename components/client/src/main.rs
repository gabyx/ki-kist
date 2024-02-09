mod error;
mod generate;
mod store;

use crate::{generate::generate_asymmetric_key_pair, store::store_key_pair};
use clap::{Args, Parser, Subcommand};
use common::result::ResultExt;
use slog::info;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kikist-cli")]
#[command(version = "0.0.1")]
#[command(about ="The client for kikist - a public/private key store.", long_about = None)]
struct Cli {
    // TODO: not yet implemented.
    #[arg(short, long, default_value_t = false)]
    non_interactive: bool,

    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Args, Debug)]
struct HostArgs {
    #[arg(long, value_name = "HOST_URL")]
    host: String,

    #[arg(short, long)]
    access_token: String,
}

#[derive(Args, Debug)]
struct PassphraseArgs {
    // If password file is not given, the user is prompted to enter one.
    #[arg(short, long)]
    passphrase: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct GenerateArgs {
    #[command(flatten)]
    pass: PassphraseArgs,
}

#[derive(Args, Debug)]
struct StoreArgs {
    #[command(flatten)]
    host: HostArgs,

    #[command(flatten)]
    pass: PassphraseArgs,
}

#[derive(Args, Debug)]
struct GetArgs {
    #[command(flatten)]
    host: HostArgs,

    #[command(flatten)]
    pass: PassphraseArgs,
}

#[derive(Args, Debug)]
struct SignArgs {
    #[command(flatten)]
    host: HostArgs,

    #[arg(short, long)]
    file: PathBuf,

    #[command(flatten)]
    pass: PassphraseArgs,
}

#[derive(Parser, Debug)]
enum Subcommands {
    Generate(GenerateArgs),
    Store(StoreArgs),
    Get(GetArgs),
    Sign(SignArgs),
}

fn main() -> Result<(), error::Error> {
    let log = common::log::create_logger(false);

    let cli = Cli::parse();
    match cli.command {
        Subcommands::Generate(args) => {
            generate_asymmetric_key_pair(&log, cli.non_interactive, &args.pass.passphrase)
                .log(&log)?;
        }
        Subcommands::Store(args) => {
            let key =
                generate_asymmetric_key_pair(&log, cli.non_interactive, &args.pass.passphrase)
                    .log(&log)?;

            store_key_pair(&log, &key, args.host.host, args.host.access_token);
        }
        Subcommands::Get(args) => (),
        Subcommands::Sign(args) => (),
    }

    Ok(())
}
