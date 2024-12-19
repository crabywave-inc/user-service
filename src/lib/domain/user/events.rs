use std::fmt::Display;
use serde::Deserialize;

pub enum UserEvent {
    Create,
    Update,
    Delete
}

pub enum UserSubscription {
    UserRegistration
}

impl Display for UserEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserEvent::Create => write!(f, "user-created"),
            UserEvent::Update => write!(f, "user-updated"),
            UserEvent::Delete => write!(f, "user-deleted"),
        }
    }
}

impl Display for UserSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSubscription::UserRegistration => write!(f, "user-registration-sub"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserRegistration {
    pub email: String,
    pub username: String,
}


