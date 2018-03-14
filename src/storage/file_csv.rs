//!
/// The CSV storage backend.

use csv;
use core::{History, Post};
use storage::StorageBackend;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind;


/// `StorageBackend` implementation using a CSV file.
///
/// Each post is stored in one line.
/// Fields are separated by `;` and are enclosed in double-quotes (`"`).
/// The `Post` fields are stored in the following order:
/// `id;time;user_agent;login;message`
///
/// For example, the following post
///
/// ```
/// use oxyboard::core::{ Post, UserPost };
///
/// let request = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
/// let post = Post::new(42, String::from("20161026120000"), request);
/// ```
///
/// is stored as `"42","20161026120000","Firefox/48.0.1","","Plop!"`.
pub struct CsvFileStorage {
	/// Directory where the data files are stored.
	dir: String,
	/// Name of the history data file.
	file: String,
}

impl CsvFileStorage {
	/// Builds a new CSV file storage backend.
	///
	/// It needs two informations, the name of the CSV file and its location (directory).
	pub fn new(p_dir: &String, p_file: String) -> CsvFileStorage {
		CsvFileStorage {
			dir : p_dir.clone(),
			file : p_file,
		}
	}


	/// Gives the full path of the backend file.
	pub fn file_path(&self) -> String {
		format!("{dir}/{file}",
				dir = self.dir,
				file = self.file)
	}
}

impl StorageBackend for CsvFileStorage {
	/// Appends the representation of a post to the CSV file.
	///
	/// The output file is opened in append mode and closed at the end of the function.
	/// It is created if needed, as is its directory path.
	fn save_post(&self, p_post: &Post) -> io::Result<&Self> {
		fs::create_dir_all(&self.dir)?;
		let mut writer = csv::Writer::from_writer(
				OpenOptions::new()
					.create(true)
					.append(true)
					.open(&self.file_path())? );

		writer.serialize(p_post).and(Ok(self)).map_err(|e| {
			match e.kind() {
				&csv::ErrorKind::Serialize(ref msg) => io::Error::new(ErrorKind::Other, format!("Failed to encode line in history file '{}': {}", self.file_path(), &msg)),
				&csv::ErrorKind::Io(ref err)        => io::Error::new(err.kind(), err.description()),
				_                                   => io::Error::new(ErrorKind::Other, "Error while saving post")
			}
		})
	}


	/// Loads a full CSV file into an `History`.
	///
	/// Returns the number of posts loaded from the file. Note that this is the number
	/// of posts read from the file. The actual number of posts in the history may be
	/// lower if it has reached its maximum size.
	fn load_history(&self, p_history: &mut History) -> io::Result<usize> {
		let mut reader = csv::ReaderBuilder::new()
				.has_headers(false)
                .from_path(self.file_path())?;

		let mut count = 0;
		for line in reader.deserialize() {
			match line {
				Ok(post) => {
					p_history.add_full_post(post);
					count += 1;
				},
				Err(err) => match err.kind() {
					&csv::ErrorKind::Serialize(ref err) => warn_msg!("Failed to parse history file '{}': {}", self.file_path(), &err),
					&csv::ErrorKind::Io(ref err)        => warn_msg!("Failed to read history file '{}': {}",  self.file_path(), &err),
					_ => {}
				}
			}
		}
		Ok(count)
	}
}
