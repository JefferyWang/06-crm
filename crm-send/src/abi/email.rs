use tonic::Status;
use tracing::warn;

use crate::{
    pb::{send_request::Msg, EmailMessage, SendRequest, SendResponse},
    NotificationService,
};

use super::{to_ts, Sender};

impl Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<EmailMessage> for Msg {
    fn from(msg: EmailMessage) -> Self {
        Msg::Email(msg)
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(msg: EmailMessage) -> Self {
        let msg: Msg = msg.into();
        Self { msg: Some(msg) }
    }
}

#[cfg(test)]
impl EmailMessage {
    pub fn fake() -> Self {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            subject: "Hello".to_string(),
            sender: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            body: "Hello, world!".to_string(),
        }
    }
}
