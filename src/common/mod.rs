extern crate iron;
extern crate mysql;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::io::prelude::*;
use std::fs::File;

use self::mysql::OptsBuilder;
use self::mysql::Pool;
use self::iron::typemap::Key;
use self::serde_yaml::Value;

pub struct MyPool;
impl Key for MyPool {
    type Value = Pool;
}

pub fn get_connection() -> Pool {
    let mut f = File::open("config/app.yaml").unwrap();

    let mut content: String = String::new();

    f.read_to_string(&mut content).unwrap();

    let config_map: Value = serde_yaml::from_str(&content.as_str()).unwrap();

    let mysql_config = config_map.get("mysql");

    let mut host: &str = "127.0.0.1";
    let mut port: u16 = 3306;
    let mut user: &str = "root";
    let mut pass: &str = "";
    let dbname: &str;

    match mysql_config {
        None => panic!("Error while reading mysql config"),
        Some(config) => {
            match config.get("host") {
                Some(_host) => {
                    host = _host.as_str().unwrap();
                }
                None => {}
            }

            match config.get("port") {
                Some(_port) => {
                    port = _port.as_u64().unwrap() as u16;
                }
                None => {}
            }

            match config.get("user") {
                Some(_user) => {
                    user = _user.as_str().unwrap();
                }
                None => {}
            }

            match config.get("pass") {
                Some(_pass) => {
                    pass = _pass.as_str().unwrap();
                }
                None => {}
            }

            match config.get("dbname") {
                Some(_dbname) => {
                    dbname = _dbname.as_str().unwrap();
                }
                None => panic!("dbname not configured"),
            }
        }
    }

    let mut builder = OptsBuilder::new();

    builder.ip_or_hostname(Some(host));
    builder.user(Some(user));
    builder.pass(Some(pass));
    builder.db_name(Some(dbname));
    builder.tcp_port(port);

    Pool::new(builder).unwrap()
}
