/*!
 * A parser for post requests.
 */

extern crate iron;
extern crate persistent;

use self::iron::status;
use history::History;
use post::UserPost;
use self::persistent::State;
use self::iron::headers::UserAgent;
use self::iron::prelude::*;
use std::io::Read;


/**
 * Constructs a new `UserPost` from an iron Request.
 */
fn make_user_post(p_request:&mut Request) -> UserPost {
	// Extract the message
	let mut body = String::new();
	p_request.body.read_to_string(&mut body).unwrap();
	let message = match extract_message(&body) {
		Some(m) => m.trim(),
		None    => ""
	};

	// Extract the user-agent
	let user_agent = match p_request.headers.get::<UserAgent>() {
		Some(ua) => ua.trim(),
		None     => "Anonyme"
	};

	UserPost {
		login     : String::new(),
		user_agent: user_agent.to_string(),
		message   : message.to_string()
	}
}


/**
 * Extracts the message from the x-form-encoded request body.
 *
 * Looks for "message=" in the request body and returns anything after it.
 */
fn extract_message(p_req_body:&String) -> Option<&str> {
	let msg_start = "message=";
	match p_req_body.rfind(&msg_start) {
		Some(s) => Some(&p_req_body[(s+msg_start.len())..]),
		None    => None
	}
}


/**
 * Handler function that manages the message reception.
 *
 * Extracts the data from the request needed to build a `UserPost` then
 * adds it to the `History` in the request's state. The id assigned to the
 * new post is then returned in the HTTP response.
 */
pub fn post_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let mut history = lock.write().unwrap();

	// Store the message and return the post id
	let post_id = history.add(make_user_post(p_request));
	Ok( Response::with(( status::Created, format!("X-Post-Id: {}", post_id) )))
}
