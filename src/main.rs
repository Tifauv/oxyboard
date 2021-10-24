extern crate clap;
#[macro_use] extern crate rocket;
extern crate oxyboard;

use clap::{ Arg, App };
use oxyboard::{ info_msg, warn_msg };
use oxyboard::config;
use oxyboard::config::{ Config, ConfigLoader, TomlConfigLoader };
use oxyboard::core::{ History, HistoryRecorder };
use oxyboard::requests::{ about, backend, board, clients_config, index };
use oxyboard::storage::{ StorageBackend, CsvFileStorage };
use rocket::fs::{ relative, FileServer };
use rocket_dyn_templates::Template;
use std::io;
use std::io::Error;
use std::path::Path;


/// Loads the configuration from the given file.
///
/// If the configuration cannot be loaded, the default configuration from
/// `oxyboard::config::default()` is returned.
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


#[launch]
fn rocket() -> _ {
	let matches = App::new("Oxyboard")
	    	.version("0.2.0")
	        .author("Olivier Serve <tifauv@gmail.com>")
	        .about("A board server written in Rust.")
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

	// Create the history storage engine
	let history_storage = CsvFileStorage::new(&config.storage.data_dir, String::from("history.csv"));

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

	// Store the history in a shared state and add it to the handlers chain
	/*
	let mut chain = Chain::new(mount(&config));
	chain.link(State::<History>::both(history));
	*/

	// Start the server
    rocket::build()
        .attach(Template::fairing())
        .manage(history)
        .mount("/", routes![index::redirect])
        .mount("/", routes![about::html])
        .mount("/", routes![board::html])
        .mount("/", routes![backend::full_xml, backend::last_xml, backend::since_xml])
        .mount("/", routes![clients_config::html])
        .mount("/res", FileServer::from(relative!("static")))
}
