pub struct Post {
	id: u32,
	timestamp: String,
	login: String,
	user_agent: String,
	message: String,
}

impl Post {
	pub fn new(p_id:u32, p_timestamp:String, p_login:String, p_user_agent:String, p_message: String) -> Post {
		Post {
			id: p_id,
			timestamp: p_timestamp,
			login: p_login,
			user_agent: p_user_agent,
			message: p_message,
		}
    }

    pub fn is_authenticated(&self) -> bool {
        self.login != ""
    }
}
