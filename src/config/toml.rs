/*!
 * Loads the configuration from a TOML file.
 */

use config::{Config, ConfigLoader};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use toml::{decode, Parser, Value};


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
	fn read_file(&self) -> io::Result<String> {
		let file = File::open(&self.file)?;
		let mut reader = BufReader::new(file);
		let mut data = String::new();
		reader.read_to_string(&mut data)?;
		Ok(data)
	}
}


impl ConfigLoader for TomlConfigLoader {
	/**
	 * Reads the configuration file and parses its content.
	 */
	fn load(&self) -> io::Result<Config> {
		let file_content = self.read_file()?;

		let mut parser = Parser::new(&file_content);
		match parser.parse() {
			Some(decoded) => {
				match decode(Value::Table(decoded)) {
					Some(config) => Ok(config),
					None => Err(Error::new(ErrorKind::InvalidData, "Invalid configuration"))
				}
			},
			None => {
				let mut error_msg = format!("Malformed configuration file '{}':", self.file);
            	for err in &parser.errors {
                	let (loline, locol) = parser.to_linecol(err.lo);
                	let (hiline, hicol) = parser.to_linecol(err.hi);
					error_msg = error_msg + &format!(" [l{}c{}-l{}c{}: {}]", loline, locol, hiline, hicol, err.desc);
				}
				Err(Error::new(ErrorKind::InvalidData, error_msg))
			}
		}
	}
}
