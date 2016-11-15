/*!
 * The data storage interfaces and facilities.
 */
use history::History;
use post::Post;
use std::io;


/**
 * Common interface for storage backends.
 *
 * This is the trait to implement if you want to add another format to save the `History`.
 */
pub trait StorageBackend {
	/**
	 * Stores a post.
	 */
	fn store_post(&self, p_post: &Post) -> io::Result<&Self>;


	/**
	 * Loads a stored history.
	 */
	fn load_history(&self, p_history: &mut History) -> io::Result<usize>;
}


// The storage backends are defined in sub-modules
pub mod file_csv;
