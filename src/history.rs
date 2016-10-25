use post::Post;
use message::Message;
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
		if self.posts.len() >= self.max_size {
			self.posts.remove(0);
		}
		
		self.last_post_id += 1;
		let post = Post::new(self.last_post_id, p_message);
		self.posts.push(post);
		self.last_post_id
	}


	pub fn iter(&self) -> Iter<Post> {
		self.posts.iter()
	}
}
