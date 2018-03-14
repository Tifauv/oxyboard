use core::history::HistoryListener;
use core::post::Post;
use std::error::Error;
use storage::StorageBackend;


/// An history listener that records all new posts using a `StorageBackend`.
pub struct HistoryRecorder<T> where T: StorageBackend {
	storage: T,
}


impl<T: StorageBackend> HistoryRecorder<T> {
	/// Initializes the recorder with a `StorageBackend`.
	pub fn new(p_storage: T) -> HistoryRecorder<T> {
		HistoryRecorder {
			storage: p_storage,
		}
	}
}


impl<T: StorageBackend> HistoryListener for HistoryRecorder<T> {
	/// Save the post using the `StorageBackend`.
	fn post_added(&self, p_post: &Post) {
		match self.storage.save_post(p_post) {
			Ok(_)  => {},
			Err(e) => warn_msg!("Failed to record post #{id}: {err}",
					id  = p_post.id(),
					err = e.description())
		}
	}


	/// Does nothing.
	fn post_removed(&self, _: &Post) {
		// Nothing to do
	}
}
