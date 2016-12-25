/*!
 * The handlers for post requests.
 */

use core::{ History, UserPost };
use iron::headers::UserAgent;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use persistent::State;
use requests::headers::{ XPostId, XPostError };
use std::result::Result;
use urlencoded::{ UrlDecodingError, UrlEncodedBody };


/**
 * Constructs a new `UserPost` from an iron Request.
 */
fn make_user_post<'a>(p_request:&mut Request) -> Result<UserPost, &'a str> {
	// Extract the user-agent
	let mut user_agent = match p_request.headers.get::<UserAgent>() {
		Some(ua) => ua.trim().to_string(),
		None     => String::from("Anonyme")
	};
	user_agent.truncate(80);

	// Extract the message
	match url_decode(p_request) {
		Ok(msg) => 	Ok(UserPost {
				login     : String::new(),
				user_agent: user_agent,
				message   : msg.trim().to_string()
			}),
		Err(..) => Err("No message in the request")
	}
}


fn url_decode(p_request: &mut Request) -> Result<String, UrlDecodingError> {
	let hashmap = p_request.get_ref::<UrlEncodedBody>()?;
	match hashmap.get("message") {
		Some(values) => Ok(values.get(0).unwrap().clone()),
		None         => Err(UrlDecodingError::EmptyQuery)
	}
}


/**
 * Handler for POST message requests.
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
	match make_user_post(p_request) {
		Ok(user_post) => match history.add_post(user_post) {
				Ok(post_id) => Ok( Response::with(( status::Created, Header(XPostId(post_id)), "Created" )) ),
				Err(error)  => Ok( Response::with(( status::InternalServerError, Header(XPostError(error.to_owned())), format!("Error: {}", error) )) )
		},
		Err(error) => Ok( Response::with(( status::BadRequest, Header(XPostError(error.to_owned())), format!("Error: {}", error) )) )
	}
}
