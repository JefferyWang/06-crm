mod abi;
mod config;
pub mod pb;

use std::{pin::Pin, sync::Arc};

pub use config::AppConfig;
use futures::Stream;
use pb::{notification_server::Notification, send_request::Msg, SendRequest, SendResponse};
use tokio::sync::mpsc;
use tonic::{async_trait, Request, Response, Status, Streaming};

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[allow(unused)]
#[derive(Clone)]
pub struct NotificationService {
    inner: Arc<NotificationServiceInner>,
}

#[allow(dead_code)]
pub struct NotificationServiceInner {
    config: AppConfig,
    sender: mpsc::Sender<Msg>,
}

#[async_trait]
impl Notification for NotificationService {
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> ServiceResult<Self::SendStream> {
        let query = request.into_inner();
        self.send(query).await
    }
}
