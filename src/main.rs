extern crate iron;
extern crate router;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use router::Router;



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
	let listenAddress = "localhost:8080";

	// Create the request router
	let mut router = Router::new();
	router.get("/backend", backend, "backend_xml");
	router.post("/post", post, "post_message");

	// Start the server
	println!("Starting board...");
	println!("  - backend: http://{}/backend", listenAddress);
	println!("  - port   : http://{}/post"   , listenAddress);
	println!("Use Ctrl-C to abort.");
	Iron::new(router).http(listenAddress).unwrap();
}
