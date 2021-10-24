use crate::core::History;
use rocket::get;
use rocket::State;
use rocket_dyn_templates::Template;


#[derive(serde::Serialize)]
struct AboutContext<'a> {
    parent: &'static str,
    board_name: &'a String
}


#[get("/about")]
pub fn html(p_history: &State<History>) -> Template {
    Template::render("about", &AboutContext {
        parent: "layout",
        board_name: &p_history.board_name()
    })
}
