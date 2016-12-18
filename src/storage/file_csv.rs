/*!
 * The CSV storage backend.
 */
use csv;
use core::{History, Post};
use storage::StorageBackend;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind;


/**
 * `StorageBackend` implementation using a CSV file.
 *
 * Each post is stored in one line.
 * Fields are separated by `;` and are enclosed in double-quotes (`"`).
 * The `Post` fields are stored in the following order:
 * `id;time;user_agent;login;message`
 *
 * For example, the following post
 *
 * ```
 * use oxyboard::core::{ Post, UserPost };
 *
 * let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
 * let post = Post::new(42, String::from("20161026120000"), request);
 * ```
 *
 * is stored as `"42","20161026120000","Firefox/48.0.1","","Plop!"`.
 */
pub struct CsvFileStorage {
	/// Directory where the data files are stored.
	dir: String,
	/// Name of the history data file.
	file: String,
}

impl CsvFileStorage {
	/**
	 * Builds a new CSV file storage backend.
	 *
	 * It needs two informations, the name of the CSV file and its location (directory).
	 */
	pub fn new(p_dir: &String, p_file: String) -> CsvFileStorage {
		CsvFileStorage {
			dir : p_dir.clone(),
			file : p_file,
		}
	}


	/**
	 * Gives the full path of the backend file.
	 */
	pub fn file_path(&self) -> String {
		format!("{dir}/{file}",
				dir = self.dir,
				file = self.file)
	}
}

impl StorageBackend for CsvFileStorage {
	/**
	 * Appends the representation of a post to the CSV file.
	 *
	 * The output file is opened in append mode and closed at the end of the function.
	 * It is created if needed, as is its directory path.
	 */
	fn save_post(&self, p_post: &Post) -> io::Result<&Self> {
		fs::create_dir_all(&self.dir)?;
		let mut writer = csv::Writer::from_writer(
				OpenOptions::new()
					.create(true)
					.append(true)
					.open(&self.file_path())? );

		writer.encode(p_post).and(Ok(self)).map_err(|e| {
			match e {
				csv::Error::Encode(msg) => io::Error::new(ErrorKind::Other, format!("Failed to encode line in history file '{}': {}", self.file_path(), msg)),
				csv::Error::Io(err)     => err,
				_                       => io::Error::new(ErrorKind::Other, "Error while saving post")
			}
		})
	}


	/**
	 * Loads a full CSV file into an `History`.
	 *
	 * Returns the number of posts loaded from the file. Note that this is the number
	 * of posts read from the file. The actual number of posts in the history may be
	 * lower if it has reached its maximum size.
	 *
	 */
	fn load_history(&self, p_history: &mut History) -> io::Result<usize> {
		let mut reader = csv::Reader::from_reader(fs::File::open(&self.file_path())? )
				.has_headers(false);

		let mut count = 0;
		for line in reader.decode() {
			match line {
				Ok(post) => {
					p_history.add_full_post(post);
					count += 1;
				},
				Err(err) => match err {
					csv::Error::Decode(msg) => warn_msg!("Failed to decode line in history file '{}': {}", self.file_path(), msg),
					csv::Error::Parse(err)  => warn_msg!("Failed to parse history file '{}': {}", self.file_path(), err),
					csv::Error::Io(err)     => warn_msg!("Failed to read history file '{}': {}", self.file_path(), err),
					_ => {}
				}
			}
		}
		Ok(count)
	}
}
