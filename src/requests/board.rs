/*!
 * The handlers for the board ui.
 */

use core::{ History, Post };
use iron::prelude::*;
use mustache::MapBuilder;
use persistent::State;
use requests::template_engine::build_response;


#[derive(RustcEncodable)]
struct PostViewModel<'a> {
	id         : u64,
	login      : &'a str,
	info       : String,
	user_agent : &'a str,
	clock      : String,
	date       : String,
	message    : &'a str,
}


impl<'a> PostViewModel<'a> {
	fn new(p_post: &Post) -> PostViewModel {
		PostViewModel {
			id         : p_post.id(),
			login      : p_post.login(),
			info       : PostViewModel::truncate_user_agent(p_post.user_agent()),
			user_agent : p_post.user_agent(),
			clock      : PostViewModel::extract_clock(p_post.time()),
			date       : PostViewModel::extract_date(p_post.time()),
			message    : p_post.message(),
		}
	}


	fn truncate_user_agent(p_user_agent: &str) -> String {
		if p_user_agent.len() > 16 {
			p_user_agent[.. 16].to_owned()
		}
		else {
			p_user_agent.to_owned()
		}
	}


	fn extract_clock(p_time: &str) -> String {
		let hours  : &str = &p_time[ 8 .. 10];
		let minutes: &str = &p_time[10 .. 12];
		let seconds: &str = &p_time[12 .. 14];
		format!("{}:{}:{}", hours, minutes, seconds)
	}


	fn extract_date(p_time: &str) -> String {
		let year : &str = &p_time[0 .. 4];
		let month: &str = &p_time[4 .. 6];
		let day  : &str = &p_time[6 .. 8];
		format!("{}/{}/{}", day, month, year)
	}
}


/**
 * Handler for GET board requests.
 *
 * Builds the HTML page and returns it.
 */
pub fn board_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let history = lock.read().unwrap();

	let data = MapBuilder::new()
		.insert_str("board_name", history.board_name())
		.insert_str("current_page_board", "class=\"active\"")
		.insert_vec("posts",      |mut builder| {
				for post in history.iter().map(|ref p| PostViewModel::new(&p)).collect::<Vec<_>>() {
					builder = builder.push(&post).unwrap();
				}
				builder
			})
		.build();

	Ok(build_response("board.html", data))
}
