/*!
 * The handlers for the about ui.
 */

use iron::prelude::*;
use iron::headers::Location;
use iron::modifiers::Header;
use iron::status;


/**
 * Handler for the default GET requests.
 *
 * Builds a redirect to the board page.
 */
pub fn default_handler(_: &mut Request) -> IronResult<Response> {
	Ok( Response::with((status::TemporaryRedirect, Header(Location("/board".to_owned())) )) )
}
