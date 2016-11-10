/*!
 * The data storage interfaces and facilities.
 */

use post::Post;
use std::io;

/**
 * Common interface for storage engines.
 */
pub trait StorageEngine {
	/**
	 * Stores a post.
	 */
	fn store(&self, p_post: &Post) -> Result<&Self, io::Error>;
}


// The storage engines are defined in sub-modules
pub mod file_csv;
