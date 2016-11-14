/*!
 * The data storage interfaces and facilities.
 */

use post::Post;
use std::io;

/**
 * Common interface for storage backends.
 */
pub trait StorageBackend {
	/**
	 * Stores a post.
	 */
	fn store(&self, p_post: &Post) -> Result<&Self, io::Error>;
}


// The storage backends are defined in sub-modules
pub mod file_csv;
