extern crate iron;

use self::iron::prelude::*;
use self::iron::{AfterMiddleware, BeforeMiddleware};

pub struct Header;

///
/// Add some header for response
///
impl AfterMiddleware for Header {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers
            .append_raw("Content-Type", b"Application/Json".to_vec());

        res.headers.append_raw(
            "Access-Control-Allow-Methods",
            b"POST, PUT, GET, OPTIONS, DELETE".to_vec(),
        );

        res.headers
            .append_raw("Access-Control-Allow-Credentials", b"true".to_vec());

        res.headers
            .append_raw("Access-Control-Allow-Headers", b"Origin, No-Cache, X-Requested-With, If-Modified-Since, Pragma, Last-Modified, Cache-Control, Expires, Content-Type, X-E4M-With".to_vec());

        res.headers
            .append_raw("Server", b"Rustont Explorer 0.0.1".to_vec());

        let origin = format!(
            "{}://{}:{}",
            req.url.scheme(),
            req.url.host(),
            req.url.port()
        ).as_str()
            .as_bytes()
            .to_vec();

        res.headers
            .append_raw("Access-Control-Allow-Origin", origin);

        Ok(res)
    }
}
