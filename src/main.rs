extern crate iron;
extern crate oxyboard;
extern crate persistent;
extern crate router;

use iron::prelude::*;
use oxyboard::config;
use oxyboard::config::ConfigLoader;
use oxyboard::config::toml_cfg::TomlConfigLoader;
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
	// Load the configuration
	let config = match TomlConfigLoader::new(String::from("oxyboard.toml")).load() {
		Ok(x) => x,
		Err(e) => {
			println!("/!\\ Configuration error: {}", e);
			println!("(i) Using default hardcoded configuration instead.");
			config::default()
		}
	};

	let listen_address = format!("{ip}:{port}",
			ip   = config.server.ip,
			port = config.server.port);

	// Create the request router
	let mut router = Router::new();
	router.get("/backend", backend::backend_handler, "backend_xml");
	router.post("/post",   post::post_handler,       "post_message");

	// Create the history storage engine
	let history_storage = CsvFileStorage::new(format!("{data_dir}/history.csv",
			data_dir = config.storage.data_dir));

	// Create the history
	let mut history = History::new(config.board.history_size);
	history.add_listener(Box::new(history_storage));

	// Store the history in the shared state
	let mut chain = Chain::new(router);
	chain.link(State::<History>::both(history));

	// Start the server
	println!("Starting board...");
	println!("  - backend: http://{}/backend", listen_address);
	println!("  - port   : http://{}/post"   , listen_address);
	println!("Use Ctrl-C to abort.");
	Iron::new(chain).http(&listen_address[..]).unwrap();
}
