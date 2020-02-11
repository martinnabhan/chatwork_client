//! # chatwork_client (English)
//!
//! The `chatwork_client` crate is an API client for [Chatwork](https://www.chatwork.com).
//!
//! The client is currently synchronous, with plans for adding an asynchronous runtime later.
//!
//! ## Initialising the client
//!
//! The client requires a Chatwork API token which can be obtained [here](http://developer.chatwork.com/).
//!
//! Example using an API token defined in an environment variable:
//! ```rust
//! let token = env::var("CHATWORK_TOKEN").expect("Please set the CHATWORK_TOKEN enironment variable.");
//! let chatwork = Chatwork::new(token);
//! ```
//!
//! ## Sending a message
//!
//! You can send a message by using the `send_message` method, specifying a room id and message.
//!
//! ```rust
//! let token = env::var("CHATWORK_TOKEN").expect("Please set the CHATWORK_TOKEN enironment variable.");
//! let chatwork = Chatwork::new(token);
//! let response = chatwork.send_message("1234", "Test message").unwrap();
//!
//! println!("response = {:?}", response);
//! ```
//!
//! # chatwork_client（日本語）
//!
//! `chatwork_client`は[チャットワーク](https://www.chatwork.com)のAPIクライエントです。
//!
//! 非同期処理はまだ対応していませんが、追加する予定です。
//!
//! ## クライエントの初期化
//!
//! クライエントを利用するにはAPIトークンが必要です。 [こちら](http://developer.chatwork.com/ja/)で申し込めます。
//!
//! 環境変数で初期化の例:
//! ```rust
//! let token = env::var("CHATWORK_TOKEN").expect("CHATWORK_TOKENを設定してください。");
//! let chatwork = Chatwork::new(token);
//! ```
//!
//! ## メッセージを送る
//!
//! `send_message`関数でメッセージを送ります。ルームIDとメッセージを指定する必要があります。
//!
//! ```rust
//! let token = env::var("CHATWORK_TOKEN").expect("CHATWORK_TOKENを設定してください。");
//! let chatwork = Chatwork::new(token);
//! let response = chatwork.send_message("1234", "テストメッセージ").unwrap();
//!
//! println!("response = {:?}", response);
//! ```

/// The Chatwork client.
pub struct Chatwork {
    client: reqwest::blocking::Client,
    token: String,
}

impl Chatwork {
    /// Initialises a new Chatwork client.
    pub fn new(token: String) -> Chatwork {
        Chatwork {
            client: reqwest::blocking::Client::new(),
            token,
        }
    }

    /// Sends a message to the specified room.
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
