extern crate iron;
extern crate persistent;
extern crate router;
extern crate serde_json;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::persistent::Read;

use datastruct::TypeResponse;
use datastruct::OntCurrent;
use datastruct::OntBlock;

use common::MyPool;
use model::{get_block_detail, get_block_height, get_transactions_for_block};

///
/// Get block height
///
pub fn block_height(request: &mut Request) -> IronResult<Response> {
    let pool = request.get::<Read<MyPool>>().unwrap();

    let currnt = get_block_height(pool);

    let mut content: TypeResponse<OntCurrent> = TypeResponse::<OntCurrent>::new();

    content.Result = Some(currnt);

    let content = self::serde_json::to_string(&content).unwrap_or("{}".to_string());

    Ok(Response::with((status::Ok, content)))
}

///
/// Get block detail
///
pub fn block_detail(req: &mut Request) -> IronResult<Response> {
    let p0 = req.get::<Read<MyPool>>().unwrap();
    let p1 = p0.clone();

    let param = req.extensions
        .get::<Router>()
        .unwrap()
        .find("param")
        .unwrap_or("");

    let mut block: OntBlock;
    let result = get_block_detail(p0, String::from(param));

    let mut content: TypeResponse<OntBlock> = TypeResponse::<OntBlock>::new();

    if result.is_some() {
        block = result.unwrap();
        block.transactions = get_transactions_for_block(p1, block.height);

        content.Result = Some(block);
    } else {
        content.Result = result;
    }

    let content = self::serde_json::to_string(&content).unwrap_or("{}".to_string());

    Ok(Response::with((status::Ok, content)))
}
