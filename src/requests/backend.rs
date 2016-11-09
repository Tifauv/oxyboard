/*!
 * The functions that handle a backend request.
 */

extern crate iron;
extern crate persistent;

use self::iron::status;
use history::History;
use self::persistent::State;
use self::iron::prelude::*;


/**
 * Handler function that returns the backend data.
 */
pub fn backend_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let history = lock.read().unwrap();

	// Build the backend
	let mut backend_xml = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
	backend_xml = backend_xml + &format!("<board site=\"{}\">\n", history.board_name());
	for post in history.iter() {
		backend_xml = backend_xml + &format!("<post id=\"{}\" time=\"{}\">", post.id(), post.time());
		backend_xml = backend_xml + &format!("<info><![CDATA[{}]]></info>", post.user_agent());
		backend_xml = backend_xml + &format!("<message><![CDATA[{}]]></message>", post.message());
		backend_xml = backend_xml + &format!("<login><![CDATA[{}]]></login>", post.login());
		backend_xml = backend_xml + &format!("</post>\n");
	}
	backend_xml.push_str("</board>");

	Ok( Response::with(( status::Ok, backend_xml )))
}
