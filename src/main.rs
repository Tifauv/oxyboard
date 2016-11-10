extern crate clap;
extern crate iron;
extern crate oxyboard;
extern crate persistent;
extern crate router;

use clap::{Arg, App};
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
 * Loads the configuration from the given file.
 *
 * If the configuration cannot be loaded, the default configuration from
 * `oxyboard::config::default()` is returned.
 */
fn load_config(p_file: &str) -> Config {
	TomlConfigLoader::new(String::from(p_file)).load()
			.and_then(|c: Config| {
				println!("\u{24d8} Configuration read from '{}'", p_file);
				Ok(c)
			})
			.or_else(|e: Error| -> io::Result<Config> {
				println!("\u{26A0} Failed to read the configuration from '{}': {}", p_file, e);
				println!("\u{24d8} Using default hardcoded configuration instead.");
				Ok(config::default())
			}).unwrap()
}


/**
 * Main function that sets the Iron server up and starts it.
 */
fn main() {
	let matches = App::new("Oxyboard")
	    	.version("0.1.0")
	        .author("Olivier Serve <tifauv@gmail.com>")
	        .about("A board server written in Rust. With clocks !")
	        .arg(Arg::with_name("config")
	            .short("c")
	            .long("config")
	            .value_name("FILE")
	            .help("Sets a custom config file")
	            .takes_value(true))
	      	.get_matches();

	// Gets a value for config if supplied by user, or defaults to "config/oxyboard.conf"
	let config_file = matches.value_of("config").unwrap_or("config/oxyboard.conf");
	let config = load_config(&config_file);

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
	println!("\u{24d8} Board '{name}' about to start listening on {ip}:{port}.",
			name = config.board.name,
			ip   = config.server.ip,
			port = config.server.port);
	println!("\u{24d8} Use Ctrl-C to abort.");
	Iron::new(chain).http((config.server.ip.as_ref(), config.server.port)).unwrap();
}
