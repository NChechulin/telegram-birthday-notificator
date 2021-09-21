use crate::user::User;
use reqwest::{Client, StatusCode};
use rand::seq::SliceRandom;

pub struct TelegramBot<'a> {
    client: Client,
    base_url: String,
    chat_ids: Vec<i32>,
    greeting_templates: &'a Vec<String>,
}

impl TelegramBot<'_> {
    pub fn new<'a>(token: &'a String, chat_ids: Vec<i32>, greeting_templates: &'a Vec<String>) -> TelegramBot<'a> {
        TelegramBot {
            client: Client::new(),
            base_url: format!("https://api.telegram.org/bot{}/sendMessage", token),
            chat_ids,
            greeting_templates,
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

    fn generate_greeting(&self, handle: &String) -> String {
        let greeting = self.greeting_templates.choose(&mut rand::thread_rng()).unwrap();
        greeting.replace("@handle", handle)
    }

    pub async fn wish_a_happy_bday(&self, user: &User) {
        let message = self.generate_greeting(&user.handle);

        for chat_id in &self.chat_ids {
            match self.send_message_to_chat(&message, *chat_id).await {
                Ok(_) => {}
                Err(msg) => eprintln!("{}", msg),
            }
        }
    }
}
