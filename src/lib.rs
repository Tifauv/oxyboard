/*!
 * # Overview
 *
 * Oxyboard is a _tribune_, a sort of web-based chat room.
 *
 * # History
 *
 * The main structure of a chat room is the `History` which contains the messages.
 *
 * ## Examples
 *
 * To create an empty history:
 *
 * ```
 * use oxyboard::core::History;
 *
 * let board_name = String::from("Oxyboard");
 * let max_size = 1024;
 * let history = History::new(&board_name, max_size);
 * ```
 *
 * To load some posts from a CSV file:
 *
 * ```
 * use oxyboard::core::History;
 * use oxyboard::storage::StorageBackend;
 * use oxyboard::storage::file_csv::CsvFileStorage;
 *
 * // Create the history
 * let board_name = String::from("Oxyboard");
 * let max_size = 1024;
 * let mut history = History::new(&board_name, max_size);
 *
 * // Load a CSV file
 * let data_dir = String::from("data");
 * let history_file = String::from("history.csv");
 * let storage = CsvFileStorage::new(&data_dir, history_file);
 * storage.load_history(&mut history);
 * ```
 *
 * # Requests
 *
 * # Configuration
 *
 */
extern crate csv;
extern crate hyper;
extern crate iron;
extern crate mustache;
extern crate persistent;
extern crate rustc_serialize;
extern crate time;
extern crate toml;
extern crate urlencoded;


/**
 * This macro prints an infomation message prefixed by Unicode character
 * 'CIRCLED LATIN SMALL LETTER I' (U+24D8).
 */
#[macro_export]
macro_rules! info_msg {
	( $tmpl: tt )               => ( println!(concat!("\u{24d8} ", $tmpl)) );
	( $tmpl: tt, $($arg: tt)* ) => ( println!(concat!("\u{24d8} ", $tmpl), $($arg)*) )
}


/**
 * This macro prints a warning message prefixed by Unicode character
 * 'WARNING SIGN' (U+26A0).
 */
#[macro_export]
macro_rules! warn_msg {
	( $tmpl: tt )               => ( println!(concat!("\u{26a0} ", $tmpl)) );
	( $tmpl: tt, $($arg: tt)* ) => ( println!(concat!("\u{26a0} ", $tmpl), $($arg)*) )
}


pub mod config;
pub mod core;
pub mod requests;
pub mod storage;
