use crate::core::{LockedHistory, UserPost};
use rocket::post;
use rocket::State;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::response::{self, Response, Responder};


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


pub struct PostId(u64);

impl<'r> Responder<'r, 'static> for PostId {
	fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
		Response::build()
			.raw_header("X-Post-Id", self.0.to_string())
			.status(Status::Created)
			.ok()
	}
}


#[post("/post", data="<p_message>")]
pub fn form(p_message: Form<Message>, p_user_agent: UserAgent<'_>, p_history: &State<LockedHistory>) -> PostId {
	let mut history = p_history.write().unwrap();

	// Process the User-Agent
	let mut user_agent = match p_user_agent.0 {
		Some(ua) => ua.trim().to_string(),
		None     => String::from("Anonymous Coward")
	};
	user_agent.truncate(80);
	
	PostId(
		history.add_post(
			UserPost {
				login     : p_message.login.trim().to_string(),
				user_agent: user_agent,
				message   : p_message.message.trim().to_string()
			}))
}
