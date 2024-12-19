use clap::Parser;

use std::sync::Arc;
use user::application::http::{HttpServer, HttpServerConfig};
use user::application::ports::messaging_ports::{MessagingPort, MessagingType, MessagingTypeImpl};
use user::domain::user::events::{UserRegistration, UserSubscription};
use user::env::Env;

pub async fn subscribe_user_created_event(messaging: Arc<MessagingTypeImpl>) -> anyhow::Result<()> {
    let subscription_name = UserSubscription::UserRegistration.to_string();

    let messaging = Arc::clone(&messaging);

    messaging
        .subscribe(&subscription_name, {
            move |message: UserRegistration| {
               async move {
                   println!("Received message: {:?}", message);

                   Ok(())
               }
            }
        })
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let messaging_port = Arc::new(MessagingTypeImpl::new(&MessagingType::PubSub, Arc::clone(&env)).await?);

    subscribe_user_created_event(Arc::clone(&messaging_port)).await?;


    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await
}
