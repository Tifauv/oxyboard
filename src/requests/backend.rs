/*!
 * The handlers for backend requests.
 */

use core::{ History, Post };
use iron::prelude::*;
use mustache::MapBuilder;
use persistent::State;
use requests::template_engine::build_response;


#[derive(RustcEncodable)]
struct PostViewModel<'a> {
	id         : u64,
	time       : &'a str,
	user_agent : &'a str,
	message    : &'a str,
	login      : &'a str,
}


impl<'a> PostViewModel<'a> {
	fn new(p_post: &Post) -> PostViewModel {
		PostViewModel {
			id         : p_post.id(),
			time       : p_post.time(),
			user_agent : p_post.user_agent(),
			message    : p_post.message(),
			login      : p_post.login(),
		}
	}
}


/**
 * Handler for GET backend requests.
 *
 * Builds the XML backend and returns it.
 */
pub fn backend_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let history = lock.read().unwrap();

		let data = MapBuilder::new()
		.insert_str("board_name", history.board_name())
		.insert_vec("posts",      |mut builder| {
				for post in history.iter().rev().map(|ref p| PostViewModel::new(&p)).collect::<Vec<_>>() {
					builder = builder.push(&post).unwrap();
				}
				builder
			})
		.build();

	Ok(build_response("backend.xml", data))
}
