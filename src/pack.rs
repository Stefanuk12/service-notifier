use reqwest::{header, Client};
use webhook::models::Message;

/// A generic pack item.
pub struct Pack {
    pub provider: String,
    pub pack: String,
    pub url: String,
}
impl Pack {
    /// Convert the pack to a markdown format.
    pub fn to_md_format(&self) -> String {
        format!("[{} - {}]({})", self.provider, self.pack, self.url)
    }

    /// Notify a webhook of the pack.
    pub async fn notify_webhook(
        &self,
        client: &Client,
        webhook: &str,
    ) -> Result<(), reqwest::Error> {
        // Construct the webhook message
        let mut message = Message::new();
        message
            .username("Service Listener")
            .content(&self.to_md_format());

        // Send the message to the webhook
        client
            .post(webhook)
            .header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&message).unwrap())
            .send()
            .await
            .map(|_| ())
    }
}

/// A trait that polls for recent packs.
pub trait PackListener {
    /// Poll for any new packs.
    fn poll(
        &mut self,
    ) -> impl std::future::Future<Output = Result<impl Iterator<Item = Pack> + Send, reqwest::Error>>
           + Send;

    /// Poll for any new packs and send them to the webhook.
    fn poll_and_notify(
        &mut self,
        webhook: Option<&str>,
    ) -> impl std::future::Future<Output = Result<u8, reqwest::Error>> + Send
    where
        Self: Send,
    {
        async move {
            let client = Client::new();
            let mut counter = 0;
            for pack in self.poll().await? {
                counter += 1;
                if let Some(webhook) = webhook {
                    pack.notify_webhook(&client, webhook).await?;
                }
            }
            Ok(counter)
        }
    }
}
