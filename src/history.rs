use post::Post;
use std::vec::Vec;

pub struct History {
	posts: Vec<Post>,
}

impl History {
    pub fn new() -> History {
        History {
            posts: Vec::new()
        }
    }
    
    
    pub fn size(&self) -> usize {
        self.posts.len()
    }
}
