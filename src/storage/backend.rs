use core::{History, Post};
use std::io;


/// Common interface for storage backends.
///
/// This is the trait to implement if you want to add another format to save the `History`.
pub trait StorageBackend {
	/// Saves a post.
	fn save_post(&self, p_post: &Post) -> io::Result<&Self>;


	/// Loads a stored history.
	fn load_history(&self, p_history: &mut History) -> io::Result<usize>;
}
