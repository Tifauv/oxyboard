/*!
 * The middleware that caches the Mustache templates.
 */

use std::collections::HashMap;
use std::io;
use std::io::{ Cursor, Error, ErrorKind };
use std::path::Path;

use iron::{ AroundMiddleware, Handler, IronError, IronResult, Request, Response };
use iron::error::HttpError;
use iron::headers::ContentType;
use iron::mime::Mime;
use iron::status;
use iron::typemap::Key;

use mustache;
use mustache::Data;


/**
 * Type used to tag the template name in the `Response` extensions.
 *
 * The type of the associated value is `String`.
 */
struct TemplateName;
impl Key for TemplateName {
	type Value = String;
}


/**
 * Type used to tag the template data in the `Response` extensions.
 *
 * The type of the associated value is `mustache::Data`.
 */
struct TemplateData;
impl Key for TemplateData {
	type Value = Data;
}


/**
 * Type used to tag the template type in the `Response` extensions.
 *
 * Tht type of the associated value is `iron::mime::Mime`.
 */
struct TemplateType;
impl Key for TemplateType {
	type Value = Mime;
}


/**
 * The wrapping `Handler` associated with a request.
 *
 * It implements a method to create a response from an instanciated template page.
 */
pub struct TemplateHandler<H: Handler> {
    template : TemplateEngine,
    handler  : H,
}

impl<H: Handler> TemplateHandler<H> {
	fn response(p_content: Cursor<Vec<u8>>, p_type: Option<&Mime>) -> Response {
		match p_type {
			Some(content_type) => Response::with((
					content_type.clone(),
					status::Ok,
					String::from_utf8(p_content.into_inner()).unwrap()
				)),

			None => Response::with((
					status::Ok,
					String::from_utf8(p_content.into_inner()).unwrap()
				)),
		}
	}
}

impl<H: Handler> Handler for TemplateHandler<H> {
	/**
	 * Retrieves the template's name and data set in the response extensions,
	 * then instanciates the template to create the final page data.
	 */
	fn handle(&self, p_request: &mut Request) -> IronResult<Response> {
        let response = self.handler.handle(p_request)?;

        if !response.extensions.contains::<TemplateName>() || !response.extensions.contains::<TemplateData>() {
			return Ok(response);
		}
        let name = response.extensions.get::<TemplateName>().unwrap();
		let data = response.extensions.get::<TemplateData>().unwrap();
		let content_type = response.extensions.get::<TemplateType>();

		match self.template.render_data(name, data) {
			Some(content) => Ok(Self::response(content, content_type)),
			None => Err(IronError::new(HttpError::Io(Error::new(ErrorKind::NotFound, "Template not found")), status::InternalServerError))
		}
    }
}


/**
 *
 */
pub struct TemplateEngine {
    templates : HashMap<String, mustache::Template>,
}

impl TemplateEngine {
    pub fn new(p_dir: &str) -> io::Result<TemplateEngine> {
		let mut templates = HashMap::new();
		let templates_path = Path::new(p_dir);

		// If the template path exists as a directory, load the template files in it
		if templates_path.is_dir() {
        	// Load all files as compiled templates indexed by their file name
			for entry in templates_path.read_dir()? {
				let entry = entry?;
    	    	match mustache::compile_path(entry.path()) {
					Ok(template) => {
						let template_name = entry.file_name().into_string().unwrap().replace(".mustache", "");
						templates.insert(template_name, template);
					},
					Err(error)   => {
						warn_msg!("The template file '{path}' could not be compiled: {error}", path = entry.path().display(), error = error)
					}
				}
			}
			info_msg!("{nb} templates loaded from directory '{dir}'.", nb = templates.len(), dir = templates_path.display());
		}
		else {
			warn_msg!("The templates directory '{dir}' does not exist !", dir = templates_path.display());
		}

        Ok(TemplateEngine {
			templates : templates,
        })
    }

    fn render_data(&self, p_name: &str, p_data: &Data) -> Option<Cursor<Vec<u8>>> {
		let mut buffer = Cursor::new(Vec::new());
		self.templates.get(p_name).map(|t| { t.render_data(&mut buffer, p_data).unwrap(); buffer } )
    }
}

impl AroundMiddleware for TemplateEngine {
	/**
	 * Wraps the given handler with a TemplateHandler.
	 */
	fn around(self, p_handler: Box<Handler>) -> Box<Handler> {
		Box::new(TemplateHandler {
            template : self,
            handler  : p_handler
        })
    }
}

pub fn build_response(p_template_name: &str, p_data: Data) -> Response {
    let mut response = Response::with((
		status::Ok,
	));

	if p_template_name.ends_with(".html") {
		response.extensions.insert::<TemplateType>(ContentType::html().0);
	}
	response.extensions.insert::<TemplateName>(p_template_name.to_string());
	response.extensions.insert::<TemplateData>(p_data);
	response
}
