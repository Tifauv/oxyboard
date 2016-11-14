use post::Post;
use storage::StorageBackend;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;


/**
 * Storage backend using a CSV file.
 */
pub struct CsvFileStorage {
	/// Directory where the data files are stored.
	dir: String,
	/// Name of the history data file.
	file: String,
}

impl CsvFileStorage {
	/**
	 * Builds a new storage engine into a CSV file.
	 */
	pub fn new(p_dir: String, p_file: String) -> CsvFileStorage {
		CsvFileStorage {
			dir : p_dir,
			file : p_file,
		}
	}


	/**
	 * Converts a post into a CSV representation.
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

impl StorageBackend for CsvFileStorage {
	/**
	 * Appends the representation of a post to the CSV file.
	 *
	 * The output file is opened in append mode and closed at the end of the function.
	 * It is created if needed, as is its directory path.
	 */
	fn store(&self, p_post: &Post) -> Result<&Self, io::Error> {
		try!(fs::create_dir_all(&self.dir));
		let full_name = format!("{dir}/{file}",
				dir = self.dir,
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
