extern crate clap;
extern crate iron;
#[macro_use]
extern crate oxyboard;
extern crate persistent;
extern crate router;

use clap::{ Arg, App };
use iron::prelude::*;
use iron::AroundMiddleware;
use oxyboard::config;
use oxyboard::config::{ Config, ConfigLoader, TomlConfigLoader };
use oxyboard::core::{ History, HistoryRecorder };
use oxyboard::requests::{ backend, board, post };
use oxyboard::requests::template_engine::TemplateEngine;
use oxyboard::storage::{ StorageBackend, CsvFileStorage };
use persistent::State;
use router::Router;
use std::io;
use std::io::Error;
use std::path::Path;


/**
 * Loads the configuration from the given file.
 *
 * If the configuration cannot be loaded, the default configuration from
 * `oxyboard::config::default()` is returned.
 */
fn load_config(p_file: &str) -> Config {
	TomlConfigLoader::new(String::from(p_file)).load()
			.and_then(|c: Config| {
				info_msg!("Configuration read from '{}'", p_file);
				Ok(c)
			})
			.or_else(|e: Error| -> io::Result<Config> {
				warn_msg!("Failed to read the configuration from '{}': {}", p_file, e);
				info_msg!("Using default hardcoded configuration instead.");
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

	// Gets a value for config if supplied by user, or defaults to "config/oxyboard.toml"
	let config_file = matches.value_of("config").unwrap_or("config/oxyboard.toml");
	let config = load_config(&config_file);

	// Create the template engine
	let template_engine = TemplateEngine::new(&config.ui.templates_dir).ok().expect("Failed to load the template files !");

	// Create the request router
	let mut router = Router::new();
	//router.get("/about",   template_engine.around(Box::new(about::about_handler)), "about_html");
	router.get("/board",   template_engine.around(Box::new(board::board_handler)), "board_html");
	router.get("/backend", backend::backend_handler, "backend_xml");
	router.post("/post",   post::post_handler,       "post_message");

	// Create the history storage engine
	let history_storage = CsvFileStorage::new(config.storage.data_dir, String::from("history.csv"));

	// Create the history
	let mut history = History::new(&config.board.name, config.board.history_size);

	// Load the history data if any
	if Path::new(&history_storage.file_path()).exists() {
		match history_storage.load_history(&mut history) {
			Ok(n)  => info_msg!("{} posts loaded from history file '{}'.", n, &history_storage.file_path()),
			Err(e) => warn_msg!("Failed to load the history file '{}': {}", &history_storage.file_path(), e)
		}
	}

	// Add the listeners
	let history_recorder = HistoryRecorder::new(history_storage);
	history.add_listener(Box::new(history_recorder));

	// Store the history in the shared state and add the template middleware
	let mut chain = Chain::new(router);
	chain.link(State::<History>::both(history));

	// Start the server
	let _server = Iron::new(chain).http((config.server.ip.as_ref(), config.server.port)).unwrap();
	info_msg!("Board '{name}' listening on {ip}:{port}. Use Ctrl-C to abort.",
			name = config.board.name,
			ip   = config.server.ip,
			port = config.server.port);
}
