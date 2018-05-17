extern crate iron;
extern crate persistent;

use super::router::Router;
use handler::*;
use common::MyPool;
use common::get_connection;

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
            port: port
        }
    }

    /// 路由和handler在这里配置
    pub fn route(&self) -> Router {
        let mut router = Router::new();
        router.add_route(("api/v1/block/height").to_string(), block_height);
        router.add_route(("api/v1/block/detail").to_string(), block_detail);

        return router;
    }

    pub fn start(&mut self) {

        let mut chain = Chain::new(self.route());

        let pool = get_connection();

        chain.link(Read::<MyPool>::both(pool));

        let server = Iron::new(chain).http(format!("{}:{}", &self.host, &self.port));

        println!("Server started as {}:{}", &self.host, &self.port);

        server.unwrap();
    }
}

