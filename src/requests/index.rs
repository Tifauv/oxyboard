use rocket::response::Redirect;
use rocket::{ get, uri };
use crate::requests::board;


#[get("/")]
pub fn redirect() -> Redirect {
    Redirect::to(uri!(board::html()))
}
