/*!
 * The history container and listener.
 */

use core::{Post, UserPost};
use iron::typemap::Key;
use time::{now, strftime};
use std::result::Result;
use std::collections::vec_deque::{VecDeque, Iter};


#[derive(RustcEncodable)]
pub struct HistoryData {
	pub board_name   : String,
	pub posts        : VecDeque<Post>,
	pub max_size     : usize,
}


/**
 * A `History` contains the current messages of a board.
 *
 * The basic structure of the `History` is a `VecDeque<Post>` which has a
 * fixed maximum size. When a `History` is full and a new message is added,
 * the oldest one is removed to maintain a constant size.
 *
 * When a `Post` is added to the history, it gains a timestamp and an id.
 * This id is generated by a sequence number held by the `History`.
 * This sequence starts at 1 and is incremented for each post.
 *
 * Finally, an `History` can signal events to listeners. Those
 * are implementors of the `HistoryListener` trait. It is usefull to hook
 * external routines.
 */
pub struct History {
	data : HistoryData,
	next_post_id : u64,
	events       : HistoryEventDispatcher
}

impl History {
	/**
	 * Constructs a new `History`.
	 *
	 * # Examples
	 *
	 * Basic usage:
	 *
	 * ```
	 * use oxyboard::core::History;
	 *
	 * let board_name = String::from("Oxyboard");
	 * let hist = History::new(&board_name, 512);
	 * assert_eq!(*hist.board_name(), String::from("Oxyboard"));
	 * assert_eq!(hist.size(), 0);
	 * ```
	 */
	pub fn new(p_name: &String, p_max_size: usize) -> History {
		History {
			data : HistoryData {
				board_name : p_name.clone(),
				posts      : VecDeque::new(),
				max_size   : p_max_size,
			},
			next_post_id : 1,
			events : HistoryEventDispatcher::new()
		}
	}


	/**
	 * Gives the name of the board represented by this history.
	 *
	 * This name is the one given to `History::new()`.
	 */
	pub fn board_name(&self) -> &String {
		&self.data.board_name
	}


	/**
	 * Gives the current number of posts in the history.
	 *
	 * This size should not be greater than `self.max_size`.
	 */
	pub fn size(&self) -> usize {
		self.data.posts.len()
	}


	/**
	 * Appends a full post at the end of the history.
	 *
	 * The given `Post` is added as-is. This function is meant to be used
	 * when loading from storage data.
	 *
	 * The main differences with `add_post` are:
	 * * it does not create the id and timestamp
	 * * the internal `next_post_id` is set to the given post's `id` + 1
	 * * no listener is called
	 * * it doesn't return anything
	 */
	pub fn add_full_post(&mut self, p_post: Post) {
		// Remove the oldest post if the history will exceed its maximum size
		if self.data.posts.len() >= self.data.max_size {
			self.events.post_removed(&self.data.posts.pop_front().unwrap());
		}

		// Add the new post
		let post_id = p_post.id();
		self.data.posts.push_back(p_post);
		self.events.post_added(&self.data.posts.back().unwrap());

		// Increment the post id counter
		self.next_post_id = post_id + 1;
	}


	/**
	 * Appends a post at the end of the history.
	 *
	 * The given `UserPost` is converted into a `Post` by giving it
	 * a (non-unique) timestamp and a unique id.
	 *
	 * The timestamp follows the datetime format YYYYmmddHHMMSS.
	 *
	 * The id is the current value of the `History::next_post_id` sequence.
	 * It is incremented after having been affected to the post.
	 *
	 * Before adding the new post, the oldest post is removed if the history is already full.
	 *
	 * On success, this function returns the id attributed to the post.
	 *
	 * # Examples
	 *
	 * Basic usage:
	 *
	 * ```
	 * use oxyboard::core::{History, UserPost};
	 *
	 * // Create a history and post message
	 * let board_name = String::from("Oxyboard");
	 * let mut hist = History::new(&board_name, 512);
	 * let post = UserPost::new(String::from(""), String::from("Firefox/48.0.1"), String::from("Plop!"));
	 *
	 * // Add the post to the history
	 * let post_id = hist.add_post(post).unwrap();
	 * assert_eq!(post_id, 1);
	 * assert_eq!(hist.size(), 1);
	 * ```
	 */
	pub fn add_post(&mut self, p_user_post: UserPost) -> Result<u64, &str> {
		// Get the current time
		let datetime = match strftime("%Y%m%d%H%M%S", &now()) {
			Ok(x) => x,
			Err(_) => return Err("Failed to format the current datetime as needed !")
		};

		// Create the new Post
		let post = Post::new(self.next_post_id, datetime, p_user_post);

		// Remove the oldest post if the history will exceed its maximum size
		if self.data.posts.len() >= self.data.max_size {
			self.events.post_removed(&self.data.posts.pop_front().unwrap());
		}

		// Add the new post
		let post_id = post.id();
		self.data.posts.push_back(post);
		self.events.post_added(&self.data.posts.back().unwrap());

		// Increment the post id counter
		self.next_post_id += 1;
		Ok(post_id)
	}


	/**
	 * Returns an immutable iterator on the posts.
	 */
	pub fn iter(&self) -> Iter<Post> {
		self.data.posts.iter()
	}


	/**
	 * Adds a `HistoryListener` to be notified of events.
	 */
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
 *
 * * `post_added`   - A new message has been added to the history.
 * * `post_removed` - The oldest message has been removed from the history.
 */
pub trait HistoryListener {
	/**
	 * Notifies a new post has been added to the history.
	 */
	fn post_added(&self, p_post: &Post);

	/**
	 * Notifies the oldest post has been removed from the history.
	 */
	fn post_removed(&self, p_post: &Post);
}


/**
 * This event dispatcher is used internally by `History` to manage
 * the listeners and dispatch the events.
 *
 * It disguises itself as a `HistoryListener` so that it doesn't need
 * a separate interface. If you need to allow several listeners, this
 * is your guy !
 */
struct HistoryEventDispatcher {
	listeners: Vec<Box<HistoryListener + Send + Sync>>
}

impl HistoryEventDispatcher {
	/**
	 * Constructs an event dispatcher.
	 */
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
	/**
	 * Dispatches the `post_added(p_post)` to every registered listener.
	 */
	fn post_added(&self, p_post: &Post) {
		for listener in &self.listeners {
			listener.post_added(p_post);
		}
	}


	/**
	 * Dispatches the `post_removed(p_post)` to every registered listener.
	 */
	fn post_removed(&self, p_post: &Post) {
		for listener in &self.listeners {
			listener.post_removed(p_post);
		}
	}
}
