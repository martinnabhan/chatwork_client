pub struct Chatwork {
    client: reqwest::blocking::Client,
    token: String,
}

impl Chatwork {
    pub fn new(token: String) -> Chatwork {
        Chatwork {
            client: reqwest::blocking::Client::new(),
            token,
        }
    }
    pub fn send_message(
        &self,
        room_id: &'static str,
        message: &'static str,
    ) -> Result<String, reqwest::Error> {
        let params = [("body", message)];
        let url = format!("https://api.chatwork.com/v2/rooms/{}/messages", room_id);
        let response = self
            .client
            .post(&url)
            .header("X-ChatWorkToken", &self.token)
            .form(&params)
            .send()?;

        response.text()
    }
}
