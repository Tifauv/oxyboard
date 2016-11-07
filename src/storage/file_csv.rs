use history::HistoryListener;
use post::Post;
use storage::engine::StorageEngine;

pub struct CsvFileStorage {
	path: String
}

impl CsvFileStorage {
	pub fn new(p_file_path: String) -> CsvFileStorage {
		CsvFileStorage {
			path : p_file_path
		}
	}
}

impl StorageEngine for CsvFileStorage {
	fn store(&self, p_post: &Post) {
		print!("Saved");
	}
}


impl HistoryListener for CsvFileStorage {
	fn post_added(&self, p_post: &Post) {
		self.store(p_post);
	}


	fn post_removed(&self, p_post: &Post) {
		// 
	}
}
