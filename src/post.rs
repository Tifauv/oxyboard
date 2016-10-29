/*!
 * A `post` in the history.
 *
 * This module contains two types, `PostRequest` and `Post`.
 * The `PostRequest` is responsible for extracting data from the HTTP request
 * whereas the `Post` represents the final data stored in the history.
 */

extern crate iron;

use self::iron::headers::UserAgent;
use self::iron::prelude::*;
use std::io::Read;


/**
 * Extracts the data from a post request.
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
 */
pub struct PostRequest {
	/// The user's login (may be empty)
	pub login: String,
	/// The user's UserAgent header value (may be empty)
	pub user_agent: String,
	/// The message content (may be empty)
	pub message: String,
}

impl PostRequest {
	/**
	 * Constructs a new `PostRequest`.
	 *
	 * This constructor is mainly targetted at tests. Real production code should use `PostRequest::from_request()`.
	 *
	 * # Examples
	 *
	 * How to build an anonymous post request:
	 *
	 * ```
	 * use oxyboard::post::PostRequest;
	 *
	 * let request = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * assert_eq!(request.login,      "");
	 * assert_eq!(request.user_agent, "Firefox/48.0.1");
	 * assert_eq!(request.message,    "Plop!");
	 * ```
	 */
	pub fn new(p_login:String, p_user_agent:String, p_message:String) -> PostRequest {
		PostRequest {
			login     : p_login,
			user_agent: p_user_agent,
			message   : p_message,
		}
	}


	/**
	 * Constructs a new `PostRequest` from an iron Request.
	 *
	 * # Examples
	 *
	 * How to build a parser:
	 *
	 * ```
	 * extern crate iron;
	 *
	 * use iron::prelude::*;
	 * use iron::status;
	 * use oxyboard::post::PostRequest;
	 *
	 * pub fn handler(p_request: &mut Request) -> IronResult<Response> {
	 *     let post_request = PostRequest::from_request(p_request);
	 *     assert_eq!(post_request.login,      "");
	 *     assert_eq!(post_request.user_agent, "Firefox/48.0.1");
	 *     assert_eq!(post_request.message,    "Plop!");
	 *     Ok( Response::with(( status::Ok )))
	 * }
	 * ```
	 */
	pub fn from_request(p_request:&mut Request) -> PostRequest {
		// Extract the message
		let mut body = String::new();
		p_request.body.read_to_string(&mut body).unwrap();
		let message = PostRequest::extract_message(&body);

		// Extract the user-agent
		let user_agent = match p_request.headers.get::<UserAgent>() {
			Some(x) => x,
			None    => "Anonyme"
		};

		PostRequest {
			login     : String::from(""),
			user_agent: String::from(user_agent.trim()),
			message   : String::from(message.trim()),
		}
    }


	/**
	 * Extracts the message from the x-form-encoded request body.
	 *
	 * Looks for "message=" in the request body and returns anything after it.
	 *
	 * # Examples
	 *
	 * With a properly formatted body (including the "message=" prefix):
	 *
	 * ```
	 * use oxyboard::post::PostRequest;
	 *
	 * let message_body = String::from("message=Plop!");
	 * let message = PostRequest::extract_message(&message_body);
	 * assert_eq!(message, "Plop!");
	 * ```
	 *
	 * With a badly formatted body (no "message=" prefix):
	 *
	 * ```
	 * use oxyboard::post::PostRequest;
	 *
	 * let message_body = String::from("Plop!");
	 * let message = PostRequest::extract_message(&message_body);
	 * assert_eq!(message, "");
	 * ```
	 */
	pub fn extract_message(p_req_body:&String) -> String {
		let msg_start = "message=";
		match p_req_body.rfind(&msg_start) {
			Some(s) => p_req_body[(s+msg_start.len())..].to_string(),
			None    => String::new(),
		}
	}
}


/**
 * Represents a post in the `History`.
 *
 * All the fields are immutableIt contains the same data as `PostRequest` and adds two metadata, `id` and `time`.
 *
 * The `id` field is a unique identifier of the post. It can be used to track responses to a
 * message.
 *
 * The `time` field is a datetime that follows the format "yyyymmddhhMMss". It is the official
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
 * The `message` field contains the message content. It is retrieved form the
 * form-encoded POST data.
 *
 * A `Post` is created from a `PostRequest` when the later is added to the `History`.
 */
pub struct Post {
	/// The post's unique identifier
	id: u32,
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
	 * This is how a `Post` is built from a `PostRequest`:
	 *
	 * ```
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let request = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * ```
	 */
	pub fn new(p_id:u32, p_datetime:String, p_parser:PostRequest) -> Post {
		Post {
			id        : p_id,
			time      : p_datetime,
			login     : p_parser.login,
			user_agent: p_parser.user_agent,
			message   : p_parser.message,
		}
	}


	/**
	 * Gives the post's identifier.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let request = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), request);
	 * assert_eq!(post.id(), 42);
	 * ```
	 */
	pub fn id(&self) -> u32 {
		self.id
	}


	/**
	 * Gives the post's creation datetime.
	 *
	 * # Examples
	 *
	 * ```
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let request = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
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
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let request = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
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
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let request = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
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
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let request = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
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
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let msg = PostRequest::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * assert!(!post.is_authenticated());
	 * ```
	 *
	 * Example for an authenticated post:
	 *
	 * ```
	 * use oxyboard::post::PostRequest;
	 * use oxyboard::post::Post;
	 *
	 * let msg = PostRequest::new(String::from("ptramo"), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 * let post = Post::new(42, String::from("20161026120000"), msg);
	 * assert!(post.is_authenticated());
	 * ```
	 */
    pub fn is_authenticated(&self) -> bool {
		!self.login.is_empty()
    }
}
