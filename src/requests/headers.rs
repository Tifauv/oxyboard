use std::fmt;

use hyper;
use hyper::header::{ HeaderFormat, Header };
use iron::headers::parsing::from_raw_str;

/**
 * `X-Post-Id` header, used by the application to return the identifier
 * of a message that has been POSTed.
 *
 * When a message is posted, it receives additional metadata such as an
 * identifier. This `id` is then used as the post reference. For clients
 * to more easily follow which post has been sent by the user, the board
 * returns the `id` attributed to a new post.
 *
 * # ABNF
 * ```plain
 * Content-Length = 1*DIGIT
 * ```
 *
 * # Example values
 * * `3495`
 *
 * # Example
 * ```
 * use hyper::header::{ Headers, XPostId };
 *
 * let mut headers = Headers::new();
 * headers.set(XPostId(1024u64));
 * ```
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPostId(pub u64);

impl Header for XPostId {
	fn header_name() -> &'static str {
        "X-Post-Id"
    }

	fn parse_header(p_raw_value: &[Vec<u8>]) -> hyper::Result<XPostId> {
        if p_raw_value.len() != 1 {
			return Err(hyper::Error::Header);
		}

        from_raw_str::<String>(&p_raw_value[0])
			.or_else(|_| Err(hyper::Error::Header))
			.and_then(|id| {
				match u64::from_str_radix(&id, 10) {
					Ok(n)  => Ok(XPostId(n)),
					Err(_) => Err(hyper::Error::Header)
				}
			})
    }
}

impl HeaderFormat for XPostId {
    fn fmt_header(&self, p_formatter: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(&self.0, p_formatter)
	}
}
