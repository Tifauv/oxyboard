extern crate iron;

use self::iron::typemap::Key;
use history::History;

impl Key for History {
	type Value = History;
}
