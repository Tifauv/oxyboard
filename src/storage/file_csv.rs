/*!
 * The CSV storage backend.
 */
use csv;
use history::History;
use post::Post;
use storage::StorageBackend;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;


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
 * use oxyboard::post::UserPost;
 * use oxyboard::post::Post;
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
	pub fn new(p_dir: String, p_file: String) -> CsvFileStorage {
		CsvFileStorage {
			dir : p_dir,
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


	/**
	 * Creates a CSV representation for a post.
	 */
	fn post_to_csv(p_post: &Post) -> String {
		format!("\"{id}\",\"{time}\",\"{info}\",\"{user}\",\"{msg}\"\n",
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
	fn store_post(&self, p_post: &Post) -> io::Result<&Self> {
		try!(fs::create_dir_all(&self.dir));
		let mut file = try!(OpenOptions::new()
				.create(true)
				.append(true)
				.open(&self.file_path()));
		let post_csv = CsvFileStorage::post_to_csv(p_post);
		try!(file.write_all(post_csv.as_bytes()));
		Ok(self)
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
		let mut reader = match csv::Reader::from_file(&self.file_path()) {
				Ok(r)  => r.has_headers(false),
				Err(e) => return match e {
					csv::Error::Encode(msg) => Err(io::Error::new(ErrorKind::Other, msg)),
					csv::Error::Decode(msg) => Err(io::Error::new(ErrorKind::Other, msg)),
					csv::Error::Index(msg)  => Err(io::Error::new(ErrorKind::Other, msg)),
					csv::Error::Parse(_)    => Err(io::Error::new(ErrorKind::Other, "Parse error")),
					csv::Error::Io(err)     => Err(err),
				}
		};

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
