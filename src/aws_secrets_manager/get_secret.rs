use aws_sdk_secretsmanager::{config::Region, meta::PKG_VERSION, types::error::InvalidParameterException, Client, Error};
use aws_config::{defaults, meta::region::RegionProviderChain, BehaviorVersion};

pub async fn get_secret(secret_name: &str, region: Option<String>, verbose: Option<bool>) -> Result<String, Error> {
    if secret_name.is_empty() {
        return Err(Error::InvalidParameterException(InvalidParameterException::builder().message("Secret name cannot be empty").build()));
    }

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    if verbose.unwrap_or(false) {
        println!("SecretsMManager client version: {}", PKG_VERSION);
        println!(
            "Region:                         {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!("Secret name:                    {}", secret_name);
        println!();
    }

    println!();

    let shared_config = defaults(BehaviorVersion::v2024_03_28())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&shared_config);
    let resp = client.get_secret_value()
        .secret_id(secret_name)
        .send()
        .await
        .map_err(|err| {
            if verbose.unwrap_or(false) {
                println!("Failed to get secret: {}", err);
            }
            err
        })?;
    Ok(resp.secret_string().unwrap_or_default().to_string())
}