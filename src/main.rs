extern crate iron;
extern crate router;
extern crate oxyboard;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use router::Router;
use oxyboard::history::History;


/**
 * Handler function that returns the backend data.
 */
fn backend(_: &mut Request) -> IronResult<Response> {
	let backend_xml = "<?xml version=\"1.0\" encoding=\"utf-8\"?><board></board>";

	Ok( Response::with(( status::Ok, backend_xml )))
}
	
	
/**
 * Handler function that manages the message reception.
 *
 * @param p_request
 *            the HTTP request
 */
fn post(p_request: &mut Request) -> IronResult<Response> {
	let mut payload = String::new();
	p_request.body.read_to_string(&mut payload).unwrap();

	Ok( Response::with(( status::Created, "post_id" )))
}


/**
 * Main function that sets the Iron server up and starts it.
 */
fn main() {
	let listen_address = "localhost:8080";

	// Create the request router
	let mut router = Router::new();
	router.get("/backend", backend, "backend_xml");
	router.post("/post", post, "post_message");

	// Start the server
	println!("Starting board...");
	println!("  - backend: http://{}/backend", listen_address);
	println!("  - port   : http://{}/post"   , listen_address);
	println!("Use Ctrl-C to abort.");
	Iron::new(router).http(listen_address).unwrap();
}
