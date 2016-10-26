/*!
 * A `post` sent by a user.
 *
 * A `Message` should have a very short life.
 * It represents the data extracted from the user's request, and
 * is then translated into a `Post` when added to the `History`.
 */

pub struct Message {
	/// The user's login, or None 
	pub login: Option<String>,
	/// The user's UserAgent header value
	pub user_agent: String,
	/// The message content
	pub message: String,
}

impl Message {
	/**
	 * Constructs a new `Message`.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::Message;
	 *
	 * let msg = Message::new(None, String::from("Firefox/48.0.1"), String::from("Hello World !"));
	 * ```
	 */
	pub fn new(p_login:Option<String>, p_user_agent:String, p_message: String) -> Message {
		Message {
			login: p_login,
			user_agent: p_user_agent,
			message: p_message,
		}
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.login != None
    }
}

pub struct Post {
	/// The post unique identifier
	pub id: u32,
	/// The post timestamp
	pub timestamp: String,
	/// The user's login, or None 
	pub login: Option<String>,
	/// The user's UserAgent header value
	pub user_agent: String,
	/// The message content
	pub message: String,
}

impl Post {
	/**
	 * Constructs a new `Post`.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::Message;
	 * use oxyboard::post::Post;
	 *
	 * let msg = Message::new(None, String::from("Firefox/48.0.1"), String::from("Hello World !"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * ```
	 */
	pub fn new(p_id:u32, p_timestamp:String, p_message:Message) -> Post {
		Post {
			id        : p_id,
			timestamp : p_timestamp,
			login     : p_message.login,
			user_agent: p_message.user_agent,
			message   : p_message.message,
		}
    }

    pub fn is_authenticated(&self) -> bool {
        self.login != None
    }
}
