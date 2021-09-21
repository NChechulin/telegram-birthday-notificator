use crate::user::User;
use reqwest::{Client, StatusCode};

pub struct TelegramBot {
    client: Client,
    base_url: String,
    chat_ids: Vec<i32>,
}

impl TelegramBot {
    pub fn new(token: &String, chat_ids: Vec<i32>) -> TelegramBot {
        TelegramBot {
            client: Client::new(),
            base_url: format!("https://api.telegram.org/bot{}/sendMessage", token),
            chat_ids,
        }
    }

    async fn send_message_to_chat(&self, message: &String, chat_id: i32) -> Result<(), String> {
        let res = self
            .client
            .get(&self.base_url)
            .query(&[
                ("chat_id", chat_id.to_string()),
                ("text", message.to_string()),
            ])
            .send()
            .await;

        match res {
            Ok(resp) => match resp.status() {
                StatusCode::OK => Ok(()),
                s => Err(format!("Request returned with error: {}", s)),
            },
            _ => Err("Could not perform the request".to_string()),
        }
    }

    pub async fn wish_a_happy_bday(&self, user: User) {
        let message: String = format!("Happy birthday, {}!", user.handle);
        for chat_id in &self.chat_ids {
            match self.send_message_to_chat(&message, *chat_id).await {
                Ok(_) => {}
                Err(msg) => eprintln!("{}", msg),
            }
        }
    }
}
