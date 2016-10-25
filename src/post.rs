use message::Message;

pub struct Post {
	pub id: u32,
	pub timestamp: String,
	pub login: String,
	pub user_agent: String,
	pub message: String,
}

impl Post {
	pub fn new(p_id:u32, p_message:Message) -> Post {
		Post {
			id        : p_id,
			timestamp : p_message.timestamp,
			login     : p_message.login,
			user_agent: p_message.user_agent,
			message   : p_message.message,
		}
    }

    pub fn is_authenticated(&self) -> bool {
        self.login != ""
    }
}
