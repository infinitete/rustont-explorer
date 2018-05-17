extern crate rustont_explorer;

use rustont_explorer::app::App;

fn main() {
    let mut app = App::new("0.0.0.0".to_string(), 3000);

    app.start();
}

