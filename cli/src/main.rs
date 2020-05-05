//
// Copyright (C) 2020 Curt Brune <curt@brune.net>
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

#[macro_use]
extern crate clap;

fn main() {
    env_logger::init();

    let arguments = parse_arguments();

    let (command, config) = match validate_arguments(&arguments) {
        Ok((command, config)) => (command, config),
        Err(()) => std::process::exit(1),
    };

    let result = unifi::station_command(command, &config);

    match result {
        Ok(()) => (),
        Err(()) => {
            log::error!("Problems executing command");
        }
    }
}

fn validate_arguments(
    arguments: &clap::ArgMatches<'static>,
) -> Result<(unifi::StationCommand, unifi::UnifiConfig), ()> {
    let command_arg = arguments.value_of("command").expect("no command argument");
    let command = match command_arg {
        "block" => unifi::StationCommand::BlockStation,
        "unblock" => unifi::StationCommand::UnblockStation,
        _ => {
            eprintln!("Command argument must be 'block' or 'unblock'");
            return Err(());
        }
    };

    let config_file = arguments.value_of("config_file").expect("no config_file");
    log::debug!("loading config file: {}", config_file);
    let mut config = unifi::parse_config(config_file)?;

    if config.user.is_none() {
        let user = arguments
            .value_of("user")
            .expect("'user' argument required if not in configuration file");
        config.user = Some(user.to_string());
    }

    if config.password.is_none() {
        let password = arguments
            .value_of("password")
            .expect("'password' argument required if not in configuration file");
        config.password = Some(password.to_string());
    }

    Ok((command, config))
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Command {
        Block,
        Unblock,
    }
}

fn parse_arguments() -> clap::ArgMatches<'static> {
    let command_argument = clap::Arg::with_name("command")
        .takes_value(true)
        .required(true)
        .short("x")
        .long("command")
        .value_name("command")
        .case_insensitive(true)
        .possible_values(&Command::variants())
        .help("Unifi command to execute");

    let config_file_argument = clap::Arg::with_name("config_file")
        .takes_value(true)
        .required(true)
        .short("c")
        .long("config")
        .value_name("config_file_path")
        .help("Path to YAML config file");

    let user_argument = clap::Arg::with_name("user")
        .takes_value(true)
        .short("u")
        .long("user")
        .value_name("user")
        .help("Unifi user to login with");

    let password_argument = clap::Arg::with_name("password")
        .takes_value(true)
        .short("p")
        .long("password")
        .value_name("password")
        .help("Unifi password to login with");

    clap::App::new("unifi-block-config")
        .version(clap::crate_version!())
        .about(format!("{} -- Utility", clap::crate_description!()).as_str())
        .author(clap::crate_authors!())
        .arg(command_argument)
        .arg(config_file_argument)
        .arg(user_argument)
        .arg(password_argument)
        .get_matches()
}
