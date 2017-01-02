/*!
 * The handlers for backend requests.
 */

use core::{ History, Post };
use iron::prelude::*;
use mustache::MapBuilder;
use persistent::State;
use requests::templates::build_response;
use router::Router;


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
 * Handles GET requests for the full backend.
 *
 * Builds the XML backend and returns it.
 */
pub fn full_backend_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let history = lock.read().unwrap();

	let data = MapBuilder::new()
		.insert_str("board_name", history.board_name())
		.insert_vec("posts",      |mut builder| {
				for post in history.iter()
						.rev()
						.map(|ref p| PostViewModel::new(&p))
						.collect::<Vec<_>>() {
					builder = builder.push(&post).unwrap();
				}
				builder
			})
		.build();

	Ok(build_response("backend.xml", data))
}


/**
 * Handles GET requests for a partial backend.
 *
 * Builds the XML backend and returns it.
 */
pub fn partial_backend_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let history = lock.read().unwrap();

	let last_id_str = p_request.extensions.get::<Router>().unwrap().find("lastId").unwrap_or("1");
	let last_id = u64::from_str_radix(last_id_str, 10).unwrap_or(1);

	let data = MapBuilder::new()
		.insert_str("board_name", history.board_name())
		.insert_vec("posts",      |mut builder| {
				for post in history.iter()
						.filter(|p| p.id() > last_id)
						.rev()
						.map(|ref p| PostViewModel::new(&p))
						.collect::<Vec<_>>() {
					builder = builder.push(&post).unwrap();
				}
				builder
			})
		.build();

	Ok(build_response("backend.xml", data))
}
