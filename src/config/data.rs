//use serde::Deserialize;

/**
 * This the root of the configuration structure.
 *
 * # Examples
 *
 * ```
 * use oxyboard::config::{ Config, ServerParams, BoardParams, StorageParams, UiParams };
 *
 * let cfg = Config {
 *         server: ServerParams {
 *             ip   : String::from("localhost"),
 *             port : 8080,
 *         },
 *
 *         board: BoardParams {
 *             name         : String::from("oxyboard"),
 *             history_size : 512,
 *         },
 *         storage: StorageParams {
 *             data_dir : String::from("data"),
 *         },
 *         ui: UiParams {
 *             templates_dir : String::from("templates"),
 *         },
 * };
 * assert_eq!(cfg.server.ip,          String::from("localhost"));
 * assert_eq!(cfg.server.port,        8080);
 * assert_eq!(cfg.board.name,         String::from("oxyboard"));
 * assert_eq!(cfg.board.history_size, 512);
 * assert_eq!(cfg.storage.data_dir,   String::from("data"));
 * assert_eq!(cfg.ui.templates_dir,   String::from("templates"));
 * ```
 */
#[derive(Debug,Deserialize)]
pub struct Config {
	pub server  : ServerParams,
	pub board   : BoardParams,
	pub storage : StorageParams,
	pub ui      : UiParams,
}


/**
 * The server parameters contain the network parameters (ip, port...)
 *
 * # Examples
 *
 * ```
 * use oxyboard::config::ServerParams;
 *
 * let server_cfg = ServerParams {
 *         ip   : String::from("localhost"),
 *         port : 8080,
 * };
 * assert_eq!(server_cfg.ip,   String::from("localhost"));
 * assert_eq!(server_cfg.port, 8080);
 * ```
 */
#[derive(Debug,Deserialize)]
pub struct ServerParams {
	pub ip   : String,
	pub port : u16,
}


/**
 * The board parameters define the served board (name, history size...)
 *
 * # Examples
 *
 * ```
 * use oxyboard::config::BoardParams;
 *
 * let board_cfg = BoardParams {
 *         name         : String::from("oxyboard"),
 *         history_size : 512,
 * };
 * assert_eq!(board_cfg.name,         String::from("oxyboard"));
 * assert_eq!(board_cfg.history_size, 512);
 * ```
 */
#[derive(Debug,Deserialize)]
pub struct BoardParams {
	pub name         : String,
	pub history_size : usize,
}


/**
 * The storage parameters define where and how the data are saved.
 *
 * # Examples
 *
 * ```
 * use oxyboard::config::StorageParams;
 *
 * let storage_cfg = StorageParams {
 *         data_dir : String::from("data"),
 * };
 * assert_eq!(storage_cfg.data_dir, String::from("data"));
 * ```
 */
#[derive(Debug,Deserialize)]
pub struct StorageParams {
	pub data_dir : String,
}


/**
 * The ui parameters define the directory of the tempaltes.
 *
 * # Examples
 *
 * ```
 * use oxyboard::config::UiParams;
 *
 * let ui_cfg = UiParams {
 *         templates_dir : String::from("templates"),
 * };
 * assert_eq!(ui_cfg.templates_dir, String::from("templates"));
 * ```
 */
#[derive(Debug,Deserialize)]
pub struct UiParams {
	pub templates_dir : String,
}
