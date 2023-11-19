use std::{println, assert_eq};

use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde_json::{Result, Value};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    base_url: String,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Auth {
        #[arg(long)]
        client_id: String,
        #[arg(long)]
        client_secret: String
    },
}

fn send_post_request(url: &String, body: String) -> Result<serde_json::Value> {
    let http_client = Client::new();
    let http_response = http_client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send();
    
    if http_response.is_ok() {
        let response = http_response.ok().unwrap();
        //println!("{:#?}", response);
        assert_eq!(response.status(), 200);

        let raw_body = response.text().unwrap();
        //println!("{:#?}", raw_body);

        let response: Value = serde_json::from_str(&raw_body)?;
        println!("{:#?}", response);

        return Ok(response)
    }  
    Ok(().into())
}


fn auth(base_url: &String, client_id: &String, client_secret: &String) {
    println!("client_id: {client_id}");
    println!("client_secret: {client_secret}");

    let uri = format!("{base_url}/oauth/token");
    let auth_request_body = format!("client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials");

    println!("Authenticating {client_id} to {uri}");
    let auth_response = send_post_request(&uri, auth_request_body).expect("ERROR: Failed to auth with {client_id} on {base_url}");
    println!("access_token: {}", auth_response["access_token"]);
    println!("expires_in: {}", auth_response["expires_in"]);
}
    

fn main() {
    let cli = Cli::parse();
    let base_url = cli.base_url;

    match &cli.command {
        Some(Commands::Auth { client_id, client_secret }) => {
            auth(&base_url, client_id, client_secret)
        }
        None => {}
    };
}
