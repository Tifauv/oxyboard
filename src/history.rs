extern crate iron;
extern crate time;

use post::{Post,UserPost};
use self::iron::typemap::Key;
use self::time::{now,strftime};
use std::result::Result;
use std::collections::vec_deque::{VecDeque,Iter};


/**
 * A History contains
 */
pub struct History {
	posts        : VecDeque<Post>,
	max_size     : usize,
	next_post_id : u32,
	events       : HistoryEventDispatcher
}

impl History {
	pub fn new(p_max_size:usize) -> History {
		History {
			posts        : VecDeque::new(),
			max_size     : p_max_size,
			next_post_id : 1,
			events       : HistoryEventDispatcher::new()
		}
	}


	pub fn size(&self) -> usize {
		self.posts.len()
	}


	pub fn add(&mut self, p_user_post:UserPost) -> Result<u32, &str> {
		// Get the current time
		let datetime = match strftime("%Y%m%d%H%M%S", &now()) {
			Ok(x) => x,
			Err(_) => return Err("Failed to format the current datetime as needed !")
		};

		// Create the new Post
		let post = Post::new(self.next_post_id, datetime, p_user_post);

		// Remove the oldest post if the history will exceed its maximum size
		if self.posts.len() >= self.max_size {
			//let oldest_post = self.posts.pop_front();
			self.events.post_removed(&self.posts.pop_front().unwrap());
		}

		// Add the new post
		let post_id = post.id();
		self.posts.push_back(post);
		self.events.post_added(&self.posts.back().unwrap());

		// Increment the post id counter
		self.next_post_id += 1;
		Ok(post_id)
	}


	pub fn iter(&self) -> Iter<Post> {
		self.posts.iter()
	}


	pub fn add_listener(&mut self, p_listener: Box<HistoryListener + Send + Sync>) {
		self.events.add_listener(p_listener);
	}
}

impl Key for History {
	type Value = History;
}


/**
 * A `HistoryListener` is the interface for listening `History` events.
 *
 * Currently, this includes:
 * *post_added: a new message has been added to the history,
 * *post_removed: the oldest message has been removed from the history.
 */
pub trait HistoryListener {
	fn post_added(&self, p_post: &Post);
	fn post_removed(&self, p_post: &Post);
}


struct HistoryEventDispatcher {
	listeners: Vec<Box<HistoryListener + Send + Sync>>
}

impl HistoryEventDispatcher {
	fn new() -> HistoryEventDispatcher {
		HistoryEventDispatcher {
			listeners : Vec::new()
		}
	}

	/**
	 * Adds a listener to this dispatcher.
	 */
	fn add_listener(&mut self, p_listener: Box<HistoryListener + Send + Sync>) {
		self.listeners.push(p_listener);
	}
}

impl HistoryListener for HistoryEventDispatcher {

	fn post_added(&self, p_post: &Post) {
		for listener in &self.listeners {
			listener.post_added(p_post);
		}
	}


	fn post_removed(&self, p_post: &Post) {
		for listener in &self.listeners {
			listener.post_removed(p_post);
		}
	}
}
