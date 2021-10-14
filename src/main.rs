use structopt::StructOpt;
extern crate dotenv;
use crate::schema::{Setting, Token};
use reqwest::{self, StatusCode};
use serde_json::json;
use std::collections::HashMap;
use std::io;

mod schema;

// structure holding the CLI args
#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long, help = "Environment to copy from, dev, stg or prd.")]
    dest: String,
    #[structopt(short, long, help = "Environment to copy into, dev, stg or prd.")]
    src: String,
    #[structopt(
        short,
        long,
        help = "Iterate through every settings and select the one to copy."
    )]
    all: bool,
    #[structopt(short, long, help = "Copy a specific setting identified by its key.")]
    key: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    // get access token
    let token = get_token().expect("Couldnt get the access token");

    // build environement hashmap to lookup
    let envs = build_envs_hash();

    // create urls for the different environements
    let source_url = envs.get(&args.src).expect("Not a valid source env");
    let dest_url = envs.get(&args.dest).expect("Not a valid dest env");
    let settings_url = "/settings/global".to_string();

    // build http client
    let client = reqwest::blocking::Client::new();

    // check if --all
    if args.all {
        if args.key.is_some() {
            panic!("*** Cant request to copy all settings and an individual key at the same time.")
        }
        let all_settings = client
            .get(format!("{}{}", source_url, &settings_url))
            .bearer_auth(&token)
            .send()?
            .json::<Vec<Setting>>()?;

        for setting in all_settings {
            println!();
            println!("---------------------------------------------------");
            println!();
            println!(
                "Do you want to copy the settings for {:?} [Y/n]?",
                &setting.key
            );
            let confirm = get_user_input();

            if confirm == "y" {
                copy_single_setting(&setting.key, &token, dest_url, source_url)?;
            }
        }
    } else {
        // if not --all
        if args.key.is_none() {
            panic!("At least --all or --key should be provided.")
        }

        copy_single_setting(&args.key.unwrap(), &token, dest_url, source_url)?;
    }

    Ok(())
}

fn copy_single_setting(
    key: &str,
    token: &str,
    dest_url: &str,
    src_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let src_setting = client
        .get(format!("{}/settings/global?key={}", src_url, key))
        .bearer_auth(&token)
        .send()?
        .json::<Setting>()?;

    // check the settings already exists in dest

    let dest_setting = client
        .get(format!("{}/settings/global?key={}", dest_url, key))
        .bearer_auth(&token)
        .send()?;

    if let StatusCode::NOT_FOUND = dest_setting.status() {
        println!(
            "Setting {} doesnt exist in dest environment, creating it first",
            key
        );

        let mut body = HashMap::new();
        body.insert("key", key);

        // create the setting using POST
        client
            .post(format!("{}/settings/global?key={}", dest_url, key))
            .bearer_auth(&token)
            .json(&body)
            .send()?
            .json::<Setting>()?;
    }

    println!(
        "Copying setting {} from {} to {} \n",
        key, src_url, dest_url
    );

    let body = json!({ "key": key, "value":  &src_setting.value});

    let response = client
        .put(format!("{}/settings/global?key={}", dest_url, key))
        .bearer_auth(&token)
        .json(&body)
        .send()?;

    match response.status() {
        StatusCode::OK => {
            println!("{} successfully copied over {}", key, dest_url);
        }
        _ => {
            eprintln!("Error!! Couldnt not copy {}", key);
        }
    }

    Ok(())
}

fn get_token() -> Result<String, reqwest::Error> {
    let url = dotenv::var("AUTH0_TOKEN_URL").expect("Missing Auth0 Url in env var");

    // get the envs vars
    dotenv::dotenv().expect("Missing .env file");
    let auth0_client_secret =
        dotenv::var("AUTH0_CLIENT_SECRET").expect("Missing Auth0_client_Secret env var");
    let auth0_client_id = dotenv::var("AUTH0_CLIENT_ID").expect("Missing Auth0_client_id env var");

    let mut params = HashMap::new();
    params.insert("audience", "https://edf-esap");
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", &auth0_client_id);
    params.insert("client_secret", &auth0_client_secret);

    let client = reqwest::blocking::Client::new();
    let res = client.post(url).form(&params).send()?;

    match res.json::<Token>() {
        Ok(data) => Ok(data.access_token),
        Err(e) => Err(e),
    }
}
fn build_envs_hash() -> HashMap<String, String> {
    let mut envs = HashMap::new();
    envs.insert(
        "local".to_string(),
        "http://localhost:5000/esap-api/v1".to_string(),
    );
    envs.insert(
        "dev".to_string(),
        "https://dev-apigw.edf-esap.com/esap-api/v1".to_string(),
    );
    envs.insert(
        "stg".to_string(),
        "https://stg-apigw.edf-esap.com/esap-api/v1".to_string(),
    );
    envs.insert(
        "prd".to_string(),
        "https://apigw.edf-esap.com/esap-api/v1".to_string(),
    );
    envs
}

fn get_user_input() -> String {
    let mut words = String::new();
    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read line.");

    let input_raw = words.to_lowercase();
    return input_raw.trim().to_string();
}
