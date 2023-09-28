//use futures::stream::StreamExt;
use tokio_stream::StreamExt;
use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::prelude::*;
use std::{sync::Arc, error::Error, process};
use clap::{App, Arg};
use std::collections::HashMap;
use serde_json::to_string_pretty;

#[derive(Debug)]
struct Config {
    subscription: String,
    keyvault_url: String,
    secrets_filter: Vec<String>,
    only_value: bool,
}

#[tokio::main]
async fn main() {
    // Get command-line arguments
    let config = get_args().unwrap();
    let _ = config.subscription;

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let client_result = SecretClient::new(&config.keyvault_url, creds);
    let client = match client_result {
        Ok(client) => client,
        Err(error) => {
            eprintln!(
                "Error creating new Azure Secret CLient {}", error);
            process::exit(1)
        }
    };

    if &config.secrets_filter.len() > &0 {
        // add secret to map
        let mut secretsmap = HashMap::new();
        for secret in &config.secrets_filter {
            let secret_result = client.clone().get(secret).await;
            match secret_result {
                Ok(s) => {
                    secretsmap.insert(secret, s.value);
                }
                Err(error) => {
                    eprintln!(
                        "Error getting Azure Secrets from CLient {}", error);
                    process::exit(1)
                }
            }
        }

        if config.only_value {
            for secret_value in secretsmap.values() {
                println!("{}", secret_value)
            }    
        } else {
            let json_string = to_string_pretty(&secretsmap).unwrap();
            println!("{}", json_string);
        }
    }
}

fn get_args() -> Result<Config, Box<dyn Error>> {
    // Define and parse command-line arguments using clap
    let matches = App::new("Template Renderer")
        .arg(
            Arg::with_name("keyvault_url")
                .short("k")
                .long("keyvault-url")
                .required(true)
                .takes_value(true)
                .help("KeyVault URL"),
        )
        .arg(
            Arg::with_name("secrets_filter")
                .short("f")
                .long("secrets-filter")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .help("Name of the secret to get value from"),
        )
        .arg(
            Arg::with_name("subscription")
                .short("s")
                .long("subscription")
                .required(false)
                .takes_value(true)
                .help("Azure Subscription ID"),
        )
        .arg(
            Arg::with_name("only_value")
                .short("v")
                .long("only-value")
                .required(false)
                .takes_value(false)
                .help("Only print the values of the secrets as plain text"),
        )
        .get_matches();

        let mut secrets: Vec<String> = vec![]; 
        if let Some(values) = matches.values_of("secrets_filter") {
            for value in values {
                secrets.push(value.to_string());
            }
        }

    Ok(Config {
        secrets_filter: secrets,
        subscription: matches.value_of("subscription").unwrap_or("").to_string(),
        keyvault_url: matches.value_of("keyvault_url").unwrap().to_string(),
        only_value: matches.is_present("only_value"),
    })
}


#[allow(dead_code)]
async fn print_all_secrets(client: SecretClient) -> azure_core::Result<()> {
    let mut stream = client.list_secrets().into_stream();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for val in r.value {
                    println!("{:?}", val.id);
                }
                break;
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    Ok(())
}