extern crate iron;
extern crate oxyboard;
extern crate persistent;
extern crate router;

use iron::prelude::*;
use iron::status;
use oxyboard::history::History;
use oxyboard::message::Message;
use persistent::State;
use router::Router;
use std::io::Read;


/**
 * Handler function that returns the backend data.
 */
fn backend(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let mut history = lock.read().unwrap();

	// Build the backend
	let mut backend_xml = "<?xml version=\"1.0\" encoding=\"utf-8\"?><board>\n".to_string();
	for post in history.iter() {
		backend_xml = backend_xml + &format!("<post time=\"{}\" id=\"{}\"><info>{}</info><message>{}</message><login>{}</login></post>\n", post.timestamp, post.id, post.user_agent, post.message, post.login);
	}
	backend_xml = backend_xml + &"</board>";

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

	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let mut history = lock.write().unwrap();

	// Store the message and return the post id
	let post_id = history.add(Message::new("20161024".to_string(), "Tifauv'".to_string(), "koinkoin".to_string(), "Broink !".to_string()));
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
