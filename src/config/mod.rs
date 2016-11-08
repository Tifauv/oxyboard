
/**
 * This the root of the configuration structure.
 */
#[derive(Debug,RustcDecodable)]
pub struct Config {
	pub server  : ServerConfig,
	pub board   : BoardConfig,
	pub storage : StorageConfig,
}


/**
 * The server parameters contain the network parameters (ip, port...)
 */
#[derive(Debug,RustcDecodable)]
pub struct ServerConfig {
	pub ip   : String,
	pub port : u64,
}


/**
 * The board parameters define the served board (name, history size...)
 */
#[derive(Debug,RustcDecodable)]
pub struct BoardConfig {
	pub name         : String,
	pub history_size : usize,
}


/**
 * The storage parameters define where and how the data are saved.
 */
#[derive(Debug,RustcDecodable)]
pub struct StorageConfig {
	pub data_dir : String,
}


/**
 * This trait defines the common interface to configuration backends.
 */
pub trait ConfigLoader {
	fn load(&self) -> Result<Config, String>;
}


// The loaders are defined in sub-modules
pub mod toml_cfg;
