extern crate iron;
extern crate serde_json;
extern crate persistent;

use self::iron::prelude::*;
use self::iron::status;
use self::persistent::Read;

use common::Action;
use common::MyPool;
use common::ResponseResult;
use common::Response as Content;
use model::get_block_height;

///
/// Get block height
///
pub fn block_height(request: &mut Request) -> IronResult<Response> {

    let pool = request.get::<Read<MyPool>>().unwrap();

    let height = get_block_height(pool).height;

    let mut content = Content::new();

    content.result = Some(ResponseResult::Height(height));
    content.desc = Some("Ok");
    content.error = Some(0);
    content.version = Some("1.0.0");

    content.set_action(Action::GetBlockHeight);

    let content =  self::serde_json::to_string(&content).unwrap_or("{}".to_string());

    Ok(Response::with((status::Ok, content)))
}

///
/// Get block detail
///
pub fn block_detail(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Block Details")))
}
