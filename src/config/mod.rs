
/**
 * This the root of the configuration structure.
 *
 * # Examples
 *
 * ```
 * use oxyboard::config::{Config,ServerParams,BoardParams,StorageParams};
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
 * };
 * assert_eq!(cfg.server.ip,          String::from("localhost"));
 * assert_eq!(cfg.server.port,        8080);
 * assert_eq!(cfg.board.name,         String::from("oxyboard"));
 * assert_eq!(cfg.board.history_size, 512);
 * assert_eq!(cfg.storage.data_dir,   String::from("data"));
 * ```
 */
#[derive(Debug,RustcDecodable)]
pub struct Config {
	pub server  : ServerParams,
	pub board   : BoardParams,
	pub storage : StorageParams,
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
#[derive(Debug,RustcDecodable)]
pub struct ServerParams {
	pub ip   : String,
	pub port : u64,
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
#[derive(Debug,RustcDecodable)]
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
#[derive(Debug,RustcDecodable)]
pub struct StorageParams {
	pub data_dir : String,
}


/**
 * Builds a default configuration.
 *
 * # Examples
 *
 * ```
 * use oxyboard::config;
 *
 * let cfg = config::default();
 * assert_eq!(cfg.server.ip,          String::from("localhost"));
 * assert_eq!(cfg.server.port,        8080);
 * assert_eq!(cfg.board.name,         String::from("oxyboard"));
 * assert_eq!(cfg.board.history_size, 512);
 * assert_eq!(cfg.storage.data_dir,   String::from("data"));
 * ```
 */
pub fn default() -> Config {
	Config {
		server: ServerParams {
			ip   : String::from("localhost"),
			port : 8080,
		},

		board : BoardParams {
			name         : String::from("oxyboard"),
			history_size : 512,
		},

		storage: StorageParams {
			data_dir : String::from("data"),
		},
	}
}


/**
 * This trait defines the common interface to configuration backends.
 */
pub trait ConfigLoader {
	fn load(&self) -> Result<Config, String>;
}


// The loaders are defined in sub-modules
pub mod toml_cfg;
