extern crate env_logger;
extern crate json;
extern crate rust_tdlib;

use json::object;
use rust_tdlib::Tdlib;
use std::env;
use std::io;

fn main() {
    env_logger::init();
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let app_id = &args[1];
    let app_hash = &args[2];

    let client = Tdlib::new();
    println!("client: {:?}", client);

    let cfg = config(app_id, app_hash);
    let req = json::stringify(cfg);

    client.send(&req[..]);

    loop {
        let _resp = client.receive(2.0);
    }
}

fn config(api_id: &str, api_hash: &str) -> json::JsonValue {
    let cfg = object!{
        "@type" => "tdlibParameters",
        "api_hash" => api_hash,
        "api_id" => api_id,
        "application_version" => "Unknown",
        "database_directory" => "tdlib",
        "device_model" => "Unknown",
        "enable_storage_optimizer" => true,
        "files_directory" => json::Null,
        "ignore_file_names" => true,
        "system_language_code" => "en",
        "system_version" => "Unknown",
        "use_chat_info_database" => true,
        "use_file_database" => true,
        "use_secret_chats" => false,
        "use_test_dc" => true
    };

    object!{
        "@type" => "setTdlibParameters",
        "parameters" => cfg
    }
}
