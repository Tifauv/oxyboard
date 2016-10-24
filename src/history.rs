use post::Post;
use std::vec::Vec;

pub struct History {
    posts: Vec<Post>,
    maxSize: usize,
}

impl History {
    pub fn new(p_maxSize:usize) -> History {
        History {
            posts: Vec::new(),
            maxSize: p_maxSize,
        }
    }
    
    
    pub fn size(&self) -> usize {
        self.posts.len()
    }
    
    
    pub fn add(&mut self, p_post:Post) -> &mut History {
        if (self.posts.len() >= self.maxSize) {
            self.posts.remove(0);
        }
        self.posts.push(p_post);
        self
    }
}
