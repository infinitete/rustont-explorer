extern crate iron;
extern crate persistent;
extern crate router;

use handler::*;
use common::MyPool;
use common::get_connection;
use middleware::Header;

use self::router::Router;
use self::iron::prelude::*;
use self::persistent::Read;
use self::iron::middleware::Chain;

pub struct App {
    pub host: String,
    pub port: u16,
    pub started: bool,
}

impl App {
    pub fn new(host: String, port: u16) -> Self {
        App {
            started: false,
            host: host,
            port: port,
        }
    }

    ///
    /// 路由和handler在这里配置
    ///
    pub fn route(&self) -> Router {
        let mut router = Router::new();
        router.get(
            ("api/v1/block/height").to_string(),
            block_height,
            "getblockheight",
        );
        router.get(
            ("api/v1/block/:param").to_string(),
            block_detail,
            "queryblock",
        );

        return router;
    }

    pub fn start(&mut self) {
        let mut chain = Chain::new(self.route());
        let pool = get_connection();
        chain.link(Read::<MyPool>::both(pool));
        chain.link_after(Header);
        let server = Iron::new(chain).http(format!("{}:{}", &self.host, &self.port));
        println!("Server served on {}:{}", &self.host, &self.port);
        server.unwrap();
    }
}
