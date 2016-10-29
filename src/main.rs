extern crate iron;
extern crate oxyboard;
extern crate persistent;
extern crate router;

use iron::prelude::*;
use iron::status;
use oxyboard::history::History;
use oxyboard::post::PostRequest;
use persistent::State;
use router::Router;


/**
 * Handler function that returns the backend data.
 */
fn backend(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let mut history = lock.read().unwrap();

	// Build the backend
	let mut backend_xml = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?><board>\n");
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


/**
 * Handler function that manages the message reception.
 *
 * @param p_request
 *            the HTTP request
 */
fn post(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let mut history = lock.write().unwrap();

	// Store the message and return the post id
	let post_id = history.add(PostRequest::from_request(p_request));
	Ok( Response::with(( status::Created, format!("X-Post-Id: {}", post_id) )))
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

	// Create the history
	let mut history = History::new(512);
	let mut chain = Chain::new(router);
	chain.link(State::<History>::both(history));

	// Start the server
	println!("Starting board...");
	println!("  - backend: http://{}/backend", listen_address);
	println!("  - port   : http://{}/post"   , listen_address);
	println!("Use Ctrl-C to abort.");
	Iron::new(chain).http(listen_address).unwrap();
}
