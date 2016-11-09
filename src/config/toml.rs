/*!
 * Loads the configuration from a TOML file.
 */
extern crate toml;

use config::{Config,ConfigLoader};
use self::toml::{Parser,Value};


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
			ip = "127.0.0.1"
			port = 8080

			[board]
			name = "oxyboard"
			history_size = 512

			[storage]
			data_dir = "data"
		"#))
	}
}


impl ConfigLoader for TomlConfigLoader {
	/**
	 * Reads the configuration file and parses its content.
	 */
	fn load(&self) -> Result<Config, String> {
		let file_content = try!(self.read_file());

		let mut parser = Parser::new(&file_content);
		match parser.parse() {
			Some(decoded) => {
				match toml::decode(Value::Table(decoded)) {
					Some(config) => Ok(config),
					None => Err(String::from("Invalid configuration"))
				}
			},
			None => {
				let mut error_msg = format!("Malformed configuration file '{}':", self.file);
            	for err in &parser.errors {
                	let (loline, locol) = parser.to_linecol(err.lo);
                	let (hiline, hicol) = parser.to_linecol(err.hi);
					error_msg = error_msg + &format!(" [l{}c{}-l{}c{}: {}]", loline, locol, hiline, hicol, err.desc);
				}
				Err(error_msg)
			}
		}
	}
}
