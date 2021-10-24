use crate::core::History;
use rocket::get;
use rocket::State;
use rocket_dyn_templates::Template;


#[derive(serde::Serialize)]
struct BoardContext<'a> {
    parent: &'static str,
    board_name: &'a String
}


#[get("/board")]
pub fn html(p_history: &State<History>) -> Template {
	// Get access to the the shared history
	//let lock = p_history.unwrap();
	//let history = lock.read().unwrap();

	Template::render("board", &BoardContext {
        parent: "layout",
        board_name: &p_history.board_name()
    })
}
