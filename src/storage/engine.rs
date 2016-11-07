/*!
 * Defines the traits used by storage engines.
 */
 
use post::Post;
use std::io;

pub trait StorageEngine {
	fn store(&self, p_post: &Post) -> Result<&Self, io::Error>;
}
