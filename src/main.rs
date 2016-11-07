extern crate iron;
extern crate oxyboard;
extern crate persistent;
extern crate router;

use iron::prelude::*;
use oxyboard::history::History;
use oxyboard::requests::backend;
use oxyboard::requests::post;
use oxyboard::storage::file_csv::CsvFileStorage;
use persistent::State;
use router::Router;


/**
 * Main function that sets the Iron server up and starts it.
 */
fn main() {
	let listen_address = "localhost:8080";

	// Create the request router
	let mut router = Router::new();
	router.get("/backend", backend::backend_handler, "backend_xml");
	router.post("/post",   post::post_handler,       "post_message");

	// Create the history storage engine
	let history_storage = CsvFileStorage::new(String::from("./history.csv"));

	// Create the history
	let mut history = History::new(512);
	history.add_listener(Box::new(history_storage));

	// Store the history in the shared state
	let mut chain = Chain::new(router);
	chain.link(State::<History>::both(history));

	// Start the server
	println!("Starting board...");
	println!("  - backend: http://{}/backend", listen_address);
	println!("  - port   : http://{}/post"   , listen_address);
	println!("Use Ctrl-C to abort.");
	Iron::new(chain).http(listen_address).unwrap();
}
