/*!
 * The handlers for the about ui.
 */

use core::History;
use iron::prelude::*;
use mustache::MapBuilder;
use persistent::State;
use requests::templates::build_response;


/**
 * Handler for GET about requests.
 *
 * Builds the HTML page and returns it.
 */
pub fn about_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let history = lock.read().unwrap();

	let data = MapBuilder::new()
		.insert_str("board_name", history.board_name().clone())
		.insert_str("current_page_about", "class=\"active\"")
		.build();

	Ok(build_response("about.html", data))
}
