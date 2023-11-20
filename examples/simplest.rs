extern crate rust_tdlib;
extern crate serde_json;

use rust_tdlib::Tdlib;
use serde_json::json;

fn main() {
    let client = Tdlib::new();

    let req = json!({"@type": "setLogVerbosityLevel", "new_verbosity_level": 2}).to_string();
    client.send(&req);

    while let Some(resp) = client.receive(5.0) {
        dbg!(resp);
    }
}
