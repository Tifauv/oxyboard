/*!
 * The handlers for the board ui.
 */

use core::History;
use std::io::Cursor;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use mustache;
use mustache::MapBuilder;
use persistent::State;


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
						<li class=\"active\"><a href=\"#\">Home</a></li>
						<li><a href=\"#about\">About</a></li>
						<li><a href=\"#contact\">Contact</a></li>
					</ul>
				</div><!--/.nav-collapse -->
			</div>
		</nav>

		<div class=\"container\">
		</div>
    </body>
    </html>").unwrap();

    // Build the template data
    let data = MapBuilder::new()
        .insert_str("board_name", &history.board_name())
        .build();

	let mut buffer = Cursor::new(Vec::new());
    template.render_data(&mut buffer, &data).unwrap();
	Ok( Response::with(( ContentType::html().0, status::Ok, String::from_utf8(buffer.into_inner()).unwrap() )))
}
