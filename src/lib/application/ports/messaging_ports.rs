use crate::env::Env;
use crate::infrastructure::messaging::pubsub::PubSubMessaging;
use anyhow::Result;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub enum MessagingType {
    #[default]
    PubSub,
}

impl ValueEnum for MessagingType {
    fn value_variants<'a>() -> &'a [Self] {
        &[MessagingType::PubSub]
    }

    fn from_str(input: &str, _ignore_case: bool) -> std::result::Result<Self, String> {
        match input {
            "pubsub" => Ok(MessagingType::PubSub),
            _ => Err("Invalid messaging type".to_string()),
        }
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            MessagingType::PubSub => Some(PossibleValue::new("pubsub")),
        }
    }
}

#[derive(Clone)]
pub enum MessagingTypeImpl {
    PubSub(PubSubMessaging),
}

impl Debug for MessagingTypeImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagingTypeImpl::PubSub(_) => write!(f, "PubSub"),
        }
    }
}

impl MessagingTypeImpl {
    pub async fn new(typ: &MessagingType, env: Arc<Env>) -> Result<Self> {
        match typ {
            MessagingType::PubSub => {
                let project_id = env.google_project_id.clone().unwrap_or_default();

                let messaging = PubSubMessaging::new(project_id).await?;
                Ok(MessagingTypeImpl::PubSub(messaging))
            }
        }
    }
}

impl MessagingPort for MessagingTypeImpl {
    async fn publish_message(&self, topic: String, message: String) -> Result<()> {
        match self {
            MessagingTypeImpl::PubSub(messaging) => messaging.publish_message(topic, message).await,
        }
    }

    async fn subscribe<F, T, Fut>(&self, topic: &str, handler: F) -> Result<()>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
        T: DeserializeOwned + Send + Sync + Debug + Clone + 'static,
    {
        println!("Type of messaging: {:?}", self);
        match self {
            MessagingTypeImpl::PubSub(messaging) => messaging.subscribe(topic, handler).await,
        }
    }
}

pub trait MessagingPort: Clone + Send + Sync + 'static {
    fn publish_message(
        &self,
        topic: String,
        message: String,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn subscribe<F, T, Fut>(
        &self,
        topic: &str,
        handler: F,
    ) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
        T: DeserializeOwned + Send + Sync + Debug + Clone + 'static;
}
