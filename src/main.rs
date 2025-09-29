use tweety_rs;
use tweety_rs::TweetyClient;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Look into using keyring crate
    // Load variables from `.env` into std::env
    dotenv().ok();

    let consumer_key = env::var("CONSUMER_KEY")?;
    let consumer_secret = env::var("CONSUMER_SECRET")?;
    let access_token = env::var("ACCESS_TOKEN")?;
    let access_secret = env::var("ACCESS_SECRET")?;

    // Create tweety client
    let tweety = tweety_rs::TweetyClient::new(
        consumer_key.as_str(),
        access_token.as_str(),
        consumer_secret.as_str(),
        access_secret.as_str(),
    );

    // Check out basic info about your X access...
    match tweety.get_user_me(None).await {
        Ok(success) => {
            println!("Success! {:?}", success);
        }
        Err(err) => {
            println!("Error!: {:?}", err);
        }
    };

    // Using my wrapper fn, for some reason...
    tweet_tweet("Hey, here's another one dude", tweety).await?;

    Ok(())
}

/// A wrapper function for posting a tweet using tweety_rs
async fn tweet_tweet(my_message: &str, tweety: TweetyClient) -> Result<(), Box<dyn std::error::Error>> {
    match tweety.post_tweet(my_message, None).await {
        Ok(success) => {
            println!("Posted tweet successfully!: {:?}", success);
        }
        Err(err ) => {
            println!("Posted tweet failed: {}", err);
        }
    }
    Ok(())
}
