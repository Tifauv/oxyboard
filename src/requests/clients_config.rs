use crate::core::History;
use rocket::get;
use rocket::State;
use rocket_dyn_templates::Template;


#[derive(serde::Serialize)]
struct ClientsConfigContext<'a> {
    parent: &'static str,
    board_name: &'a String
}


#[get("/clients/config")]
pub fn html(p_history: &State<History>) -> Template {
    Template::render("clients_config", &ClientsConfigContext {
        parent: "layout",
        board_name: &p_history.board_name()
    })
}
