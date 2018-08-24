extern crate rust_tdlib;

use rust_tdlib::Tdlib;

fn main() {
    println!("Hello, world!");

    {
        let client = Tdlib::create();
        // let client = rust_tdlib::client_create();
        println!("client: {:?}", client);
        //TODO doesn't drop here for some reason
    }
    // unsafe { td_json_client_destroy(client); }
    // println!("client: {:?}", client);
    println!("ok")
}
