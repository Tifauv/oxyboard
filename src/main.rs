extern crate iron;
extern crate oxyboard;
extern crate persistent;
extern crate router;

use iron::prelude::*;
use oxyboard::config;
use oxyboard::config::{Config,ConfigLoader};
use oxyboard::config::toml::TomlConfigLoader;
use oxyboard::history::History;
use oxyboard::requests::backend;
use oxyboard::requests::post;
use oxyboard::storage::file_csv::CsvFileStorage;
use persistent::State;
use router::Router;
use std::io;
use std::io::Error;


/**
 * Main function that sets the Iron server up and starts it.
 */
fn main() {
	// Load the configuration
	let config_file = "config/oxyboard.toml";
	let config = TomlConfigLoader::new(String::from(config_file)).load()
			.and_then(|c: Config| {
				println!("\u{24d8} Configuration read from '{}'", config_file);
				Ok(c)
			})
			.or_else(|e: Error| -> io::Result<Config> {
				println!("\u{26A0} Failed to read the configuration from '{}': {}", config_file, e);
				println!("\u{24d8} Using default hardcoded configuration instead.");
				Ok(config::default())
			}).unwrap();

	// Create the request router
	let mut router = Router::new();
	router.get("/backend", backend::backend_handler, "backend_xml");
	router.post("/post",   post::post_handler,       "post_message");

	// Create the history storage engine
	let history_storage = CsvFileStorage::new(config.storage.data_dir, String::from("history.csv"));

	// Create the history
	let mut history = History::new(&config.board.name, config.board.history_size);
	history.add_listener(Box::new(history_storage));

	// Store the history in the shared state
	let mut chain = Chain::new(router);
	chain.link(State::<History>::both(history));

	// Start the server
	let listen_address = format!("{ip}:{port}",
			ip   = config.server.ip,
			port = config.server.port);
	println!("\u{24d8} Starting board '{}'..."        , config.board.name);
	println!("\u{24d8}  - backend: http://{}/backend", listen_address);
	println!("\u{24d8}  - port   : http://{}/post"   , listen_address);
	println!("\u{24d8} Use Ctrl-C to abort.");
	Iron::new(chain).http((config.server.ip.as_ref(), config.server.port)).unwrap();
}
