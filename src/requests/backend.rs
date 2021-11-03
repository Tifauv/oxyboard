//!
/// The handlers for backend requests.

use crate::core::{LockedHistory, Post};
use rocket::get;
use rocket::State;
use rocket_dyn_templates::Template;


#[derive(serde::Serialize)]
struct PostViewModel<'a> {
	id         : u64,
	time       : &'a str,
	user_agent : &'a str,
	message    : &'a str,
	login      : &'a str,
}

impl<'a> PostViewModel<'a> {
	fn new(p_post: &Post) -> PostViewModel {
		PostViewModel {
			id         : p_post.id(),
			time       : p_post.time(),
			user_agent : p_post.user_agent(),
			message    : p_post.message(),
			login      : p_post.login(),
		}
	}
}


#[derive(serde::Serialize)]
struct BackendContext<'a> {
    parent: &'static str,
    board_name: &'a String,
	posts: Vec<PostViewModel<'a>>
}


/// Handles GET requests for the full backend.
///
/// Builds the XML backend and returns it.
#[get("/backend")]
pub fn full_xml(p_history: &State<LockedHistory>) -> Template {
	let history = p_history.read().unwrap();

	let mut posts_view = Vec::new();
	for post in history.iter()
		.rev()
		.map(|ref p| PostViewModel::new(&p))
		.collect::<Vec<_>>() {
		posts_view.push(post);
	}

	Template::render("backend", &BackendContext {
		parent: "layout",
		board_name: &history.board_name(),
		posts: posts_view
	})
}


/// Handles GET requests for a backend containing the last n messages.
///
/// Uses a :size URL parameter.
#[get("/backend/last/<p_size>")]
pub fn last_xml(p_size: usize, p_history: &State<LockedHistory>) -> Template {
	let history = p_history.read().unwrap();

	let mut posts_view = Vec::new();
	for post in history.iter()
		.rev()
		.take(p_size)
		.map(|ref p| PostViewModel::new(&p))
		.collect::<Vec<_>>() {
		posts_view.push(post);
	}

	Template::render("backend", &BackendContext {
		parent: "layout",
		board_name: &history.board_name(),
		posts: posts_view
	})
}


/// Handles GET requests for a backend since a given post id.
///
/// Uses a :lastId URL parameter.
///
/// Builds the XML backend containing only the posts having an id
/// greater than the given one. If no :lastId parameter is found,
/// uses "1" as the lastId.
///
/// @returns the backend
#[get("/backend/since/<p_post_id>")]
pub fn since_xml(p_post_id: u64, p_history: &State<LockedHistory>) -> Template {
	let history = p_history.read().unwrap();

	let mut posts_view = Vec::new();
	for post in history.iter()
		.filter(|p| p.id() > p_post_id)
		.rev()
		.map(|ref p| PostViewModel::new(&p))
		.collect::<Vec<_>>() {
		posts_view.push(post);
	}

	Template::render("backend", &BackendContext {
		parent: "layout",
		board_name: &history.board_name(),
		posts: posts_view
	})
}
