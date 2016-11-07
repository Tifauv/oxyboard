/*!
 * Defines the traits used by storage engines.
 */
 
use post::Post;

pub trait StorageEngine {
	fn store(&self, p_post: &Post);
}
