extern crate rust_tdlib;

fn main() {
    println!("Hello, world!");

    let client;
    {
        client = rust_tdlib::client_create();
        println!("client: {:?}", client);
        //TODO doesn't drop here for some reason
    }
    // unsafe { td_json_client_destroy(client); }
    println!("client: {:?}", client);
}
