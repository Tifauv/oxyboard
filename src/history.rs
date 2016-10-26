extern crate iron;
extern crate time;

use post::Message;
use post::Post;
use self::iron::typemap::Key;
use self::time::{now,strftime};
use std::slice::Iter;
use std::vec::Vec;


pub struct History {
	posts: Vec<Post>,
	max_size: usize,
	last_post_id: u32,
}

impl History {
	pub fn new(p_max_size:usize) -> History {
		History {
			posts: Vec::new(),
			max_size: p_max_size,
			last_post_id: 1,
		}
	}


	pub fn size(&self) -> usize {
		self.posts.len()
	}


	pub fn add(&mut self, p_message:Message) -> u32 {
		// Get the current time
		let datetime = match strftime("%Y%m%d%H%M%S", &now()) {
			Ok(x) => x,
			Err(_) => panic!("Failed to format the current datetime as needed !"),
		};

		// Increment the post id counter
		self.last_post_id += 1;

		// Create the new Post
		let post = Post::new(self.last_post_id, datetime, p_message);

		// Remove the oldest post if the history will exceed its maximum size
		if self.posts.len() >= self.max_size {
			self.posts.remove(0);
		}

		// Add the new post and return its id
		self.posts.push(post);
		self.last_post_id
	}


	pub fn iter(&self) -> Iter<Post> {
		self.posts.iter()
	}
}

impl Key for History {
	type Value = History;
}
