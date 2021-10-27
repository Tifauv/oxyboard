use crate::core::{History, UserPost};
use rocket::post;
use rocket::State;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest, Outcome};
use std::sync::Mutex;


pub struct UserAgent<'r>(Option<&'r str>);

#[derive(Debug)]
pub enum UserAgentError {
	Missing
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
	type Error = ();
	
	async fn from_request(p_request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		Outcome::Success(UserAgent(p_request.headers().get_one("User-Agent")))
	}
}


#[derive(FromForm)]
pub struct Message {
	login: String,
	message: String
}


#[post("/post", data="<p_message>")]
pub fn form(p_message: Form<Message>, p_user_agent: UserAgent<'_>, p_history: State<Mutex<History>>) -> Status {
	let mut history = p_history.lock().expect("lock history");

	// Process the User-Agent
	let mut user_agent = match p_user_agent.0 {
		Some(ua) => ua.trim().to_string(),
		None     => String::from("Anonymous Coward")
	};
	user_agent.truncate(80);
	
	match history.add_post(
			UserPost {
				login     : p_message.login,
				user_agent: user_agent,
				message   : p_message.message
			}) {
		Ok(post_id) => Status::Created,             // TODO Add the X-Post-Id header
		Err(error)  => Status::InternalServerError, // TODO Add the X-Post-Error
	}
}
