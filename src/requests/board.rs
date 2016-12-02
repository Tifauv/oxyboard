/*!
 * The handlers for the board ui.
 */

use core::{History, Post};
use std::io::Cursor;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use mustache;
use persistent::State;


#[derive(RustcEncodable)]
struct PostViewModel<'a> {
	login      : &'a str,
	user_agent : String,
	clock      : String,
	date       : String,
	message    : &'a str,
}


impl<'a> PostViewModel<'a> {
	fn new(p_post: &Post) -> PostViewModel {
		PostViewModel {
			login      : p_post.login(),
			user_agent : PostViewModel::truncate_user_agent(p_post.user_agent()),
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

#[derive(RustcEncodable)]
struct BoardViewModel<'a> {
	board_name : &'a str,
	posts      : Vec<PostViewModel<'a>>,
}


impl<'a> BoardViewModel<'a> {
	fn new(p_history: &History) -> BoardViewModel {
		BoardViewModel {
			board_name : p_history.board_name(),
			posts      : p_history.iter().map(|ref p| PostViewModel::new(&p)).collect::<Vec<_>>(),
		}
	}
}


/**
 * Handler for GET backend requests.
 *
 * Builds the XML backend and returns it.
 */
pub fn board_handler(p_request: &mut Request) -> IronResult<Response> {
	// Get access to the the shared history
	let lock = p_request.get::<State<History>>().unwrap();
	let history = lock.read().unwrap();

	// Build the backend
    let template = mustache::compile_str("<!DOCTYPE html>
    <html lang=\"fr\">
    <head>
		<meta charset=\"utf-8\">
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
		<title>{{board_name}}</title>
		<link rel=\"stylesheet\" href=\"https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css\" integrity=\"sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u\" crossorigin=\"anonymous\">
		<!--[if lt IE 9]>
		<script src=\"https://oss.maxcdn.com/html5shiv/3.7.3/html5shiv.min.js\"></script>
		<script src=\"https://oss.maxcdn.com/respond/1.4.2/respond.min.js\"></script>
		<![endif]-->
		<style>
		#content { margin-top: 71px }
		</style>
    </head>
    <body>
		<nav class=\"navbar navbar-inverse navbar-fixed-top\">
			<div class=\"container\">
				<div class=\"navbar-header\">
					<button type=\"button\" class=\"navbar-toggle collapsed\" data-toggle=\"collapse\" data-target=\"#navbar\" aria-expanded=\"false\" aria-controls=\"navbar\">
						<span class=\"sr-only\">Toggle navigation</span>
						<span class=\"icon-bar\"></span>
						<span class=\"icon-bar\"></span>
						<span class=\"icon-bar\"></span>
					</button>
					<a class=\"navbar-brand\" href=\"#\">{{board_name}}</a>
				</div>
				<div id=\"navbar\" class=\"collapse navbar-collapse\">
					<ul class=\"nav navbar-nav\">
						<li class=\"active\"><a href=\"#\">Tribune</a></li>
						<li><a href=\"#config\">Config</a></li>
						<li><a href=\"#\">Login</a></li>
					</ul>
				</div><!--/.nav-collapse -->
			</div>
		</nav>

		<div id=\"content\" class=\"container\">
			<div id=\"board\" class=\"col-sm-10 panel panel-default\">
				<div class=\"panel-body\">
					{{#posts}}
					<div class=\"row\">
						<div class=\"col-md-2 text-right\">
							<span class=\"post-author\">{{user_agent}}</span>
						</div>
						<div class=\"col-md-1\" title=\"{{date}}\">
							<span class=\"post-time\">{{clock}}</span>
						</div>
						<div class=\"col-md-9 text-justify\">
							<span class=\"post-message\">{{message}}</span>
						</div>
					</div>
					{{/posts}}
				</div>
			</div>
			<div id=\"post\" class=\"col-sm-10\">
				<form name=\"post-form\" class=\"form-inline\" method=\"post\" action=\"/post\">
					<div class=\"form-group\">
						<label class=\"sr-only\" for=\"message\">Message</label>
						<div class=\"input-group\">
							<div class=\"input-group-addon\">Anonyme</div>
							<input type=\"text\" class=\"form-control\" id=\"message\" name=\"message\" placeholder=\"Entrez votre message ici\">
						</div>
						<button type=\"submit\" class=\"btn btn-primary\">Envoyer</button>
					</div>
				</form>
			</div>
		</div>
    </body>
    </html>").unwrap();

	let mut buffer = Cursor::new(Vec::new());
	let data = BoardViewModel::new(&history);
    template.render(&mut buffer, &data).unwrap();
	Ok( Response::with(( ContentType::html().0, status::Ok, String::from_utf8(buffer.into_inner()).unwrap() )))
}
