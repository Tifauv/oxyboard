/*!
 * A `post` sent by a user.
 *
 * This module contains two types, `Message` and `Post` which are the building
 * blocks of a board.
 */

/**
 * Represents the data extracted from a post request.
 *
 * The `login` field is the account name of the author if the user is
 * authenticated.
 *
 * The `user_agent` field is the UserAgent HTTP header of the post request. It is used
 * as a lousy author identification mechanism for unauthenticated posts. As it
 * can be modified at will by browser extensions or dedicated clients, it is
 * easy to set and modify, even if it cannot provide a verified identity.
 *
 * The `message` field contains the message content. It is retrieved form the
 * form-encoded POST data.
 *
 * A `Message` should be short-lived.
 */
pub struct Message {
	/// The user's login (may be empty)
	pub login: String,
	/// The user's UserAgent header value (should not be empty)
	pub user_agent: String,
	/// The message content (may be empty)
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
	 *
	 * # Panics
	 * If `p_user_agent` is empty.
	 */
	pub fn new(p_login:String, p_user_agent:String, p_message: String) -> Message {
		assert!(!p_user_agent.is_empty());

		Message {
			login: p_login,
			user_agent: p_user_agent,
			message: p_message,
		}
    }
}


/**
 * Represents a post in the `History`.
 *
 * It contains the same data as `Message` and adds two metadata, `id` and `time`.
 *
 * The `id` field is a unique identifier of the post. It can be used to track responses to a
 * message.
 *
 * The `time` field is a datetime that follows the format "yyyymmddhhMMss". It is the official
 * timestamp of the post. It can also be used to track responses to a message.
 *
 * A `Post` is created from a `Message` when the later is added to the `History`.
 */
pub struct Post {
	/// The post's unique identifier
	pub id: u32,
	/// The datetime when the post was added to the history
	pub time: String,
	/// The user's login (may be empty)
	pub login: String,
	/// The user's UserAgent header value
	pub user_agent: String,
	/// The message content (may be empty)
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
	 * Example for an anonymous post:
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
	 * Example for an authenticated post:
	 *
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
