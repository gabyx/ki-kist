mod error;
mod generate;
mod get;
mod password;
mod sign;
mod store;
mod verify;

use crate::{
    generate::generate_asymmetric_key_pair, get::get_key_pair,
    store::store_key_pair,
};
use clap::{Args, Parser};
use common::result::ResultExt;
use sign::sign_file;
use std::path::PathBuf;
use uuid::Uuid;
use verify::verify_file;

#[derive(Parser)]
#[command(name = "kikist-cli")]
#[command(version = "0.0.1")]
#[command(about ="The client for kikist - a public/private key store.", long_about = None)]
struct Cli {
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
struct UserArgs {
    #[arg(short, long)]
    user_id: String,
}

#[derive(Args, Debug)]
struct PassphraseArgs {
    // If password file is not given, the user is prompted to enter one.
    #[arg(short, long)]
    passphrase_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct FileArgs {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(long)]
    file_signature: Option<PathBuf>,
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
    user: UserArgs,

    #[command(flatten)]
    pass: PassphraseArgs,
}

#[derive(Args, Debug)]
struct GetArgs {
    #[command(flatten)]
    host: HostArgs,

    #[command(flatten)]
    user: UserArgs,

    #[arg(short, long)]
    key_id: Uuid,
}

#[derive(Args, Debug)]
struct SignArgs {
    #[command(flatten)]
    host: HostArgs,

    #[command(flatten)]
    user: UserArgs,

    #[arg(short, long)]
    key_id: Uuid,

    #[command(flatten)]
    pass: PassphraseArgs,

    #[command(flatten)]
    file: FileArgs,
}

#[derive(Args, Debug)]
struct VerifyArgs {
    #[command(flatten)]
    host: HostArgs,

    #[command(flatten)]
    user: UserArgs,

    #[arg(short, long)]
    key_id: Uuid,

    #[command(flatten)]
    file: FileArgs,
}

#[derive(Parser, Debug)]
enum Subcommands {
    Generate(GenerateArgs),
    Store(StoreArgs),
    Get(GetArgs),
    Sign(SignArgs),
    Verify(VerifyArgs),
}

fn main() -> Result<(), error::Error> {
    let log = common::log::create_logger(false);

    let cli = Cli::parse();
    match cli.command {
        Subcommands::Generate(args) => {
            generate_asymmetric_key_pair(
                &log,
                cli.non_interactive,
                &args.pass.passphrase_file,
            )
            .log(&log)?;
        }
        Subcommands::Store(args) => {
            let key = generate_asymmetric_key_pair(
                &log,
                cli.non_interactive,
                &args.pass.passphrase_file,
            )
            .log(&log)?;

            store_key_pair(
                &log,
                &args.host.host,
                &args.host.access_token,
                &args.user.user_id,
                &key,
            )
            .log(&log)?;
        }
        Subcommands::Get(args) => {
            get_key_pair(
                &log,
                &args.host.host,
                &args.host.access_token,
                &args.user.user_id,
                &args.key_id,
            )
            .log(&log)?;
        }
        Subcommands::Sign(args) => {
            let key = get_key_pair(
                &log,
                &args.host.host,
                &args.host.access_token,
                &args.user.user_id,
                &args.key_id,
            )
            .log(&log)?;

            sign_file(
                &log,
                cli.non_interactive,
                &key,
                &args.pass.passphrase_file,
                &args.file.file,
                args.file.file_signature,
            )
            .log(&log)?
        }
        Subcommands::Verify(args) => {
            let key = get_key_pair(
                &log,
                &args.host.host,
                &args.host.access_token,
                &args.user.user_id,
                &args.key_id,
            )
            .log(&log)?;

            verify_file(&log, &key, &args.file.file, args.file.file_signature)
                .log(&log)?
        }
    }

    Ok(())
}
