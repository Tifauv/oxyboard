extern crate iron;
extern crate time;

use post::{Post,UserPost};
use self::iron::typemap::Key;
use self::time::{now,strftime};
use std::slice::Iter;
use std::vec::Vec;


pub struct History {
	posts: Vec<Post>,
	max_size: usize,
	next_post_id: u32,
}

impl History {
	pub fn new(p_max_size:usize) -> History {
		History {
			posts: Vec::new(),
			max_size: p_max_size,
			next_post_id: 1,
		}
	}


	pub fn size(&self) -> usize {
		self.posts.len()
	}


	pub fn add(&mut self, p_user_post:UserPost) -> u32 {
		// Get the current time
		let datetime = match strftime("%Y%m%d%H%M%S", &now()) {
			Ok(x) => x,
			Err(_) => panic!("Failed to format the current datetime as needed !"),
		};

		// Create the new Post
		let post = Post::new(self.next_post_id, datetime, p_user_post);

		// Remove the oldest post if the history will exceed its maximum size
		if self.posts.len() >= self.max_size {
			self.posts.remove(0);
		}

		// Add the new post
		let post_id = post.id();
		self.posts.push(post);

		// Increment the post id counter
		self.next_post_id += 1;
		post_id
	}


	pub fn iter(&self) -> Iter<Post> {
		self.posts.iter()
	}
}

impl Key for History {
	type Value = History;
}
