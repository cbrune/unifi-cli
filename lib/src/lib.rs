//
// Copyright (C) 2020 Curt Brune <curt@brune.net>
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

use std::fs;
use std::u8;

use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, ClientBuilder, RequestBuilder};

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnifiConfig {
    pub base_url: String,

    pub site: String,

    pub accept_invalid_certs: bool,

    pub user: Option<String>,

    pub password: Option<String>,

    pub client_macs: Vec<String>,
}

#[derive(Debug)]
pub enum StationCommand {
    BlockStation,
    UnblockStation,
}

pub fn parse_config(config_file_path: &str) -> Result<UnifiConfig, ()> {
    let config_file = match fs::File::open(config_file_path) {
        Ok(config_file) => config_file,
        Err(error) => {
            log::error!("error opening config file {}: {}", config_file_path, error);
            return Err(());
        }
    };

    let config = match serde_yaml::from_reader::<_, UnifiConfig>(config_file) {
        Ok(config) => config,
        Err(error) => {
            log::error!(
                "Error parsing config file {}: {:?}",
                config_file_path,
                error
            );
            return Err(());
        }
    };

    log::debug!("Config: {:?}", config);

    for mac in &config.client_macs {
        match parse_mac_addr(&mac) {
            Ok(()) => (),
            Err(()) => {
                log::error!("Error parsing config file {}:", config_file_path);
                log::error!("Badly formed MAC address: {}", mac);
                return Err(());
            }
        }
        log::debug!("mac: {}", mac);
    }

    Ok(config)
}

fn parse_mac_addr(mac: &str) -> Result<(), ()> {
    // break mac into 6 strings by ":"
    let v: Vec<&str> = mac.split(':').collect();
    if v.len() != 6 {
        return Err(());
    }

    for item in v {
        match u8::from_str_radix(&item, 16) {
            Ok(_) => (),
            Err(_) => return Err(()),
        }
    }

    Ok(())
}

fn make_request(client: &Client, uri: &str, json_body: &String) -> RequestBuilder {
    let request = client
        .post(uri)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(json_body.clone());
    log::debug!("  request: {:?}", request);

    request
}

fn send_request(request: RequestBuilder) -> Result<(), ()> {
    match request.send() {
        Ok(mut result) => {
            if result.status().is_success() {
                log::debug!("success:\n{}", result.text().unwrap());
                Ok(())
            } else if result.status().is_server_error() {
                log::error!("HTTP server error: {:?}", result.status());
                Err(())
            } else {
                log::error!("HTTP respsonse: {:?}", result.status());
                Err(())
            }
        }
        Err(e) => {
            log::warn!("Sending request failed: {:?}", e);
            Err(())
        }
    }
}

#[derive(Serialize, Debug)]
struct LoginData {
    username: String,
    password: String,
}

fn api_login(client: &Client, config: &UnifiConfig) -> Result<(), ()> {
    let login_uri = format!("{}/api/login", config.base_url);
    let login_data = LoginData {
        username: match config.user {
            Some(ref user) => user.to_owned(),
            None => panic!("'user' field must be set in configuration by now"),
        },
        password: match config.password {
            Some(ref password) => password.to_owned(),
            None => panic!("'password' field must be set in configuration by now"),
        },
    };
    let json_login_data = serde_json::to_string(&login_data).unwrap();
    log::debug!("  login_data: {:?}", json_login_data);

    let request = make_request(client, &login_uri, &json_login_data);
    send_request(request)?;

    Ok(())
}

fn build_client(config: &UnifiConfig) -> Client {
    ClientBuilder::new()
        .tcp_nodelay()
        .gzip(false)
        .http1_title_case_headers()
        .cookie_store(true)
        .danger_accept_invalid_certs(config.accept_invalid_certs)
        .build()
        .unwrap()
}

#[derive(Serialize, Debug)]
struct StationCommandData {
    cmd: String,
    mac: String,
}

pub fn station_command(command: StationCommand, config: &UnifiConfig) -> Result<(), ()> {
    let client = build_client(config);

    api_login(&client, config)?;

    let station_uri = format!("{}/api/s/{}/cmd/stamgr", config.base_url, config.site);

    for mac in &config.client_macs {
        let station_command = StationCommandData {
            cmd: match command {
                StationCommand::BlockStation => "block-sta".to_owned(),
                StationCommand::UnblockStation => "unblock-sta".to_owned(),
            },
            mac: mac.to_owned(),
        };

        let json_station_command = serde_json::to_string(&station_command).unwrap();
        log::debug!("  station_command: {:?}", json_station_command);

        log::info!("{:?}: {}", command, mac);
        let request = make_request(&client, &station_uri, &json_station_command);
        send_request(request)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
