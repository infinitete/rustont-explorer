extern crate mysql;
extern crate iron;
extern crate serde;
extern crate serde_json;

use self::mysql::OptsBuilder;
use self::mysql::Pool;
use self::iron::typemap::Key;

#[derive(Serialize, Deserialize)]
pub enum ResponseResult {
    Height(u64),
    Data(String),
    None,
}

#[derive(Serialize, Deserialize)]
pub struct Response<'a> {
    pub action: Option<&'a str>,
    pub desc: Option<&'a str>,
    pub error: Option<u16>,
    pub result: Option<ResponseResult>,
    pub version: Option<&'a str>,
}

impl<'a> Response<'a> {
    pub fn new() -> Response<'a> {
        Response {
            action: None,
            desc: None,
            error: None,
            result: None,
            version: None,
        }
    }

    pub fn set_action(&mut self, action: Action) {
        match action {
            Action::GetBlockHeight => {
                self.action = Some("getbloclheight");
            }

            Action::GetBlockDetails => self.action = Some("getblockdetails"),
        }
    }
}

///
/// Actions
///
pub enum Action {
    GetBlockHeight,
    GetBlockDetails,
}

pub struct MyPool;
impl Key for MyPool { type Value = Pool; }

pub fn get_connection() -> Pool {
    let mut builder = OptsBuilder::new();

    builder.ip_or_hostname(Some("192.168.1.19"));
    builder.user(Some("root"));
    builder.pass(Some("123456"));
    builder.db_name(Some("explorer"));
    builder.tcp_port(19336);

    Pool::new(builder).unwrap()
}
