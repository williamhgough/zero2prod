use crate::domain::{subscriber_email::SubscriberEmail, subscriber_name::SubscriberName};

#[derive(Debug, Clone)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
