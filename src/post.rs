/*!
 * The representations of a message and its metadata.
 *
 * This module contains two types, `UserPost` and `Post`.
 * The `UserPost` is responsible for extracting data from the HTTP request
 * whereas the `Post` represents the final data stored in the history.
 */


 /**
  * Contains the data extracted from a post request.
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
  * All these fields are immutable.
  */
 pub struct UserPost {
 	/// The user's login (may be empty)
 	pub login: String,
 	/// The user's UserAgent header value (may be empty)
 	pub user_agent: String,
 	/// The message content (may be empty)
 	pub message: String,
 }

 impl UserPost {
 	/**
 	 * Constructs a new `UserPost`.
 	 *
 	 * This constructor is mainly targetted at tests. Real production code should use `UserPost::from_request()`.
 	 *
 	 * # Examples
 	 *
 	 * How to build an anonymous post request:
 	 *
 	 * ```
 	 * use oxyboard::post::UserPost;
 	 *
 	 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
 	 * assert_eq!(request.login,      "");
 	 * assert_eq!(request.user_agent, "Firefox/48.0.1");
 	 * assert_eq!(request.message,    "Plop!");
 	 * ```
 	 */
 	pub fn new(p_login: String, p_user_agent: String, p_message: String) -> UserPost {
 		UserPost {
 			login      : p_login,
 			user_agent : p_user_agent,
 			message    : p_message,
 		}
 	}
 }


/**
 * Represents a post in the `History`.
 *
 * A `Post` adds two fields to a `UserPost`, `id` and `time`.
 *
 * The `id` field is a unique identifier of the post. It can be used to track responses to a
 * message.
 *
 * The `time` field is a (non-unique) datetime that follows the format "YYYYmmddHHMMSS". It is the official
 * timestamp of the post. It can also be used to track responses to a message.
 *
 * The `login` field is the account name of the author if the user is
 * authenticated.
 *
 * The `user_agent` field is the UserAgent HTTP header of the post request. It is used
 * as a lousy author identification mechanism for unauthenticated posts. As it
 * can be modified at will by browser extensions or dedicated clients, it is
 * easy to set and modify, even if it cannot provide a verified identity.
 *
 * The `message` field contains the message content.
 *
 * All these fields are immutable.
 */
pub struct Post {
	/// The post's unique identifier
	id: u64,
	/// The datetime when the post was added to the history
	time: String,
	/// The user's login (may be empty)
	login: String,
	/// The user's UserAgent header value
	user_agent: String,
	/// The message content (may be empty)
	message: String,
}

impl Post {
	/**
	 * Constructs a new `Post`.
	 *
	 * # Examples
	 *
	 * This is how a `Post` is built from a `UserPost`:
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * ```
	 */
	pub fn new(p_id: u64, p_datetime: String, p_parser: UserPost) -> Post {
		Post {
			id         : p_id,
			time       : p_datetime,
			login      : p_parser.login,
			user_agent : p_parser.user_agent,
			message    : p_parser.message
		}
	}


	/**
	 * Gives the post's identifier.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * assert_eq!(post.id(), 42);
	 * ```
	 */
	pub fn id(&self) -> u64 {
		self.id
	}


	/**
	 * Gives the post's creation datetime.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * assert_eq!(post.time(), "20161026120000");
	 * ```
	 */
	pub fn time(&self) -> &String {
		&self.time
	}


	/**
	 * Gives the post's author login (if it was authenticated).
	 *
	 * An empty string means the post was send by an anonymous user.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * assert_eq!(post.login(), "");
	 * ```
	 */
	pub fn login(&self) -> &String {
		&self.login
	}


	/**
	 * Gives the post's author UserAgent.
	 *
	 * This may be helpfull to identify the author.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * assert_eq!(post.user_agent(), "Firefox/48.0.1");
	 * ```
	 */
	pub fn user_agent(&self) -> &String {
		&self.user_agent
	}


	/**
	 * Gives the post's message.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * assert_eq!(post.message(), "Plop!");
	 * ```
	 */
	pub fn message(&self) -> &String {
		&self.message
	}


	/**
	 * Tells whether the post is from an authenticated user or not
	 *
	 * # Examples
	 *
	 * Example for an anonymous post:
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let msg = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * assert!(!post.is_authenticated());
	 * ```
	 *
	 * Example for an authenticated post:
	 *
	 * ```
	 * use oxyboard::post::UserPost;
	 * use oxyboard::post::Post;
	 *
	 * let msg = UserPost::new(String::from("ptramo"), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * assert!(post.is_authenticated());
	 * ```
	 */
    pub fn is_authenticated(&self) -> bool {
		!self.login.is_empty()
    }
}
