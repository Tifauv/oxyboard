use history::HistoryListener;
use post::Post;
use storage::engine::StorageEngine;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::error::Error;
use std::io;


pub struct CsvFileStorage {
	path: String,
	file: String,
}

impl CsvFileStorage {
	/**
	 * Builds a new storage engine into a CSV file.
	 */
	pub fn new(p_path: String, p_file: String) -> CsvFileStorage {
		CsvFileStorage {
			path : p_path,
			file : p_file,
		}
	}


	/**
	 * Converts a post to its CSV description.
	 */
	fn post_to_csv(p_post: &Post) -> String {
		format!("\"{id}\";\"{time}\";\"{info}\";\"{user}\";\"{msg}\"\n",
				id   = p_post.id(),
				time = p_post.time(),
				info = p_post.user_agent(),
				user = p_post.login(),
				msg  = p_post.message())
	}
}

impl StorageEngine for CsvFileStorage {
	fn store(&self, p_post: &Post) -> Result<&Self, io::Error> {
		try!(fs::create_dir_all(&self.path));
		let full_name = format!("{path}/{file}",
				path = self.path,
				file = self.file);
		let mut file = try!(OpenOptions::new()
				.create(true)
				.append(true)
				.open(&full_name));
		let post_csv = CsvFileStorage::post_to_csv(p_post);
		try!(file.write_all(post_csv.as_bytes()));
		Ok(self)
	}
}


impl HistoryListener for CsvFileStorage {
	fn post_added(&self, p_post: &Post) {
		match self.store(p_post) {
			Ok(_)  => {},
			Err(e) => println!("Error: {}", e.description())
		}
	}


	fn post_removed(&self, _: &Post) {
		//
	}
}
