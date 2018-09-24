### Prerequisites

Have [tdlib](https://github.com/tdlib/td) built and `libtdjson.so` installed into library search path (here [are](https://unix.stackexchange.com/questions/22926/where-do-executables-look-for-shared-objects-at-runtime) [some](http://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html) [links](http://man7.org/linux/man-pages/man8/ld.so.8.html)), usually `.so` file should be copied somewhere like `/usr/lib`, `/usr/lib64` or `/lib64`.

### Using and running

Here follows a working example

`main.rs`:

    extern crate env_logger;
    extern crate json;
    extern crate rust_tdlib;

    use json::object;
    use rust_tdlib::Tdlib;
    use std::env;

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


Get your `app_id` and `app_hash` at https://my.telegram.org/apps and run

    $ cargo run <app_id> <app_hash>
