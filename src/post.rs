/*!
 * A `post` sent by a user.
 *
 * A `Message` should have a very short life.
 * It represents the data extracted from the user's request, and
 * is then translated into a `Post` when added to the `History`.
 */

pub struct Message {
	/// The user's login (may be empty) 
	pub login: String,
	/// The user's UserAgent header value (should not be empty)
	pub user_agent: String,
	/// The message content (shoud not be empty)
	pub message: String,
}

impl Message {
	/**
	 * Constructs a new `Message`.
	 *
	 * # Examples
	 *
	 * How to build an anonymous message:
	 *
	 * ```
	 * use oxyboard::post::Message;
	 *
	 * let msg = Message::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Hello World !"));
	 * ```
	 *
	 * How to build an authenticated message:
	 *
	 * ```
	 * use oxyboard::post::Message;
	 *
	 * let msg = Message::new(String::from("ptramo"), String::from("Firefox/48.0.1"), String::from("Hello World !"));
	 * ```
	 */
	pub fn new(p_login:String, p_user_agent:String, p_message: String) -> Message {
		assert!(true, p_user_agent != "");
		assert!(true, p_message != "");

		Message {
			login: p_login,
			user_agent: p_user_agent,
			message: p_message,
		}
    }
}

pub struct Post {
	/// The post's unique identifier
	pub id: u32,
	/// The datetime when the post was added to the history
	pub time: String,
	/// The user's login (may be empty) 
	pub login: String,
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
	 * This is how a `Post` is built from a `Message`:
	 *
	 * ```
	 * use oxyboard::post::Message;
	 * use oxyboard::post::Post;
	 *
	 * let msg = Message::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Hello World !"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * ```
	 */
	pub fn new(p_id:u32, p_datetime:String, p_message:Message) -> Post {
		Post {
			id        : p_id,
			time      : p_datetime,
			login     : p_message.login,
			user_agent: p_message.user_agent,
			message   : p_message.message,
		}
    }


	/**
	 * Tells whether the post is from an authenticated user or not
	 *
	 * # Examples
	 *
	 * Example for an anonymous post :
	 *
	 * ```
	 * use oxyboard::post::Message;
	 * use oxyboard::post::Post;
	 *
	 * let msg = Message::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Hello World !"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * assert!(!post.is_authenticated());
	 * ```
	 *
	 * Example for an authenticated post :
	 * ```
	 * use oxyboard::post::Message;
	 * use oxyboard::post::Post;
	 *
	 * let msg = Message::new(String::from("ptramo"), String::from("Firefox/48.0.1"), String::from("Hello World !"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * assert!(post.is_authenticated());
	 * ```
	 */
    pub fn is_authenticated(&self) -> bool {
		!self.login.is_empty()
    }
}
