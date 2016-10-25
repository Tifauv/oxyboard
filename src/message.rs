pub struct Message {
	pub timestamp: String,
	pub login: String,
	pub user_agent: String,
	pub message: String,
}

impl Message {
	pub fn new(p_timestamp:String, p_login:String, p_user_agent:String, p_message: String) -> Message {
		Message {
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
