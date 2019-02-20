use std::path::{Path, PathBuf};

use clap::{
    App, AppSettings, Arg, Error,
    ErrorKind::{InvalidSubcommand, MissingRequiredArgument, MissingSubcommand},
    Result, SubCommand,
};

#[derive(Debug)]
pub enum Args {
    Backup(BackupArgs),
    Restore(RestoreArgs),
}

#[derive(Debug)]
struct BackupArgs {
    pub from: Box<Path>,
    pub to: Box<Path>,
}

#[derive(Debug)]
struct RestoreArgs {
    pub structure_file: Box<Path>,
}

pub fn arg_parse() -> Result<Args> {
    let matches = App::new("bumpg")
        .version("0.1")
        .author("Shogo Takata <pineapplehunter.danie@gmail.com>")
        .about("This is a backup system for multiple projects using git")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("backup")
                .about("backup")
                .arg(
                    Arg::with_name("from")
                        .short("f")
                        .long("from")
                        .takes_value(true)
                        .value_name("FROM_DIR")
                        .required(true)
                        .help("the path to the directory you want to backup."),
                )
                .arg(
                    Arg::with_name("to")
                        .short("t")
                        .long("to")
                        .takes_value(true)
                        .value_name("TO_DIR")
                        .required(true)
                        .help("the path to where you want to store the backups"),
                ),
        )
        .subcommand(
            SubCommand::with_name("restore").about("restore").arg(
                Arg::with_name("structure-file")
                    .short("s")
                    .long("structure-file")
                    .takes_value(true)
                    .value_name("STRUCTURE_FILE")
                    .required(true)
                    .help("the path to the structure file of your backup"),
            ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("backup") => {
            let matches = matches.subcommand_matches("backup").unwrap();
            Ok(Args::Backup(BackupArgs {
                from: PathBuf::from(
                    matches
                        .value_of("from")
                        .ok_or(Error::with_description(
                            "could not find argument with from",
                            MissingRequiredArgument,
                        ))?
                        .to_owned(),
                )
                .into_boxed_path(),
                to: PathBuf::from(
                    matches
                        .value_of("to")
                        .ok_or(Error::with_description(
                            "could not fine argument with from",
                            MissingRequiredArgument,
                        ))?
                        .to_owned(),
                )
                .into_boxed_path(),
            }))
        }
        Some("restore") => {
            let matches = matches.subcommand_matches("restore").unwrap();
            Ok(Args::Restore(RestoreArgs {
                structure_file: PathBuf::from(matches.value_of("structure-file").ok_or(
                    Error::with_description(
                        "could not find argument with structure file",
                        MissingRequiredArgument,
                    ),
                )?)
                .into_boxed_path(),
            }))
        }
        Some(_) => Err(Error::with_description(
            "undefined subcommand found",
            InvalidSubcommand,
        )),
        None => Err(Error::with_description(
            "no sub command found",
            MissingSubcommand,
        )),
    }
}
