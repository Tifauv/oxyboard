/*!
 * Loads the configuration from a TOML file.
 */
extern crate toml;

use config::{Config,ConfigLoader};


pub struct TomlConfigLoader {
	file : String,
}


impl TomlConfigLoader {
	pub fn new(p_config_file: String) -> TomlConfigLoader {
		TomlConfigLoader {
			file : p_config_file
		}
	}


	/**
	 * Reads the content of the configuration file.
	 */
	fn read_file(&self) -> Result<String, &str> {
		Ok(String::from(r#"
			[server]
			ip: 127.0.0.1
			port: 8080

			[board]
			name: oxyboard
			history_size: 512

			[storage]
			data_dir: .
		"#))
	}
}


impl ConfigLoader for TomlConfigLoader {
	/**
	 * Reads the configuration file and parses its content.
	 */
	fn load(&self) -> Result<Config, &str> {
		let config_data = try!(self.read_file());
		match toml::decode_str(&config_data) {
			Some(decoded) => Ok(decoded),
			None => Err("Failed to read the configuration")
		}
	}
}
