use std::{env, time::Duration};

use rust_tdlib::Tdlib;
use serde_json::{json, Value};
use tokio::sync::mpsc::{Receiver, Sender};

#[tokio::main]
async fn main() {
    let client = Tdlib::new();

    let (auth_tx, auth_rx) = tokio::sync::mpsc::channel(5);

    let c = client.clone();

    // Handle authorization state updates AND wait for update with @extra = 1
    let handle = tokio::spawn(async move {
        while let Some(update) = c.receive(5.0) {
            handle_update(update, &auth_tx, 1).await;
        }
    });

    let req = json!({"@type": "setLogVerbosityLevel", "new_verbosity_level": 2}).to_string();
    client.send(&req);

    let auth_rx = handle_authorization_state(client.clone(), auth_rx).await;

    // Send getMe request with @extra = 1
    let req = json!({"@type": "getMe", "@extra": 1});
    client.send(&serde_json::to_string(&req).unwrap());

    tokio::time::sleep(Duration::from_secs(1)).await;
    let req = json!({"@type": "close"});
    client.send(&serde_json::to_string(&req).unwrap());

    handle_authorization_state(client, auth_rx).await;

    handle.await.unwrap();
}

fn ask_user(string: &str) -> String {
    println!("{}", string);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

async fn handle_update(data: String, auth_tx: &Sender<String>, extra: i32) {
    let value: Value = serde_json::from_str(&data).unwrap();
    let update = value.as_object().expect("update is not an object");
    let update_type = update
        .get("@type")
        .expect("update doesn't have '@type' key")
        .as_str()
        .expect("'@type' is not a string");

    match update_type {
        "updateAuthorizationState" => {
            let auth_state = update
                .get("authorization_state")
                .unwrap()
                .as_object()
                .unwrap()
                .get("@type")
                .unwrap()
                .as_str()
                .unwrap();
            auth_tx.send(auth_state.to_string()).await.unwrap();
        }
        "updateOption" => {}
        "error" => panic!("{}", data),
        _ => {}
    }

    if let Some(e) = update.get("@extra") {
        if e == extra {
            println!("{}", value);
        }
    }
}

async fn handle_authorization_state(
    client: Tdlib,
    mut auth_rx: Receiver<String>,
) -> Receiver<String> {
    while let Some(state) = auth_rx.recv().await {
        // dbg!(&state);
        match state.as_str() {
            "authorizationStateWaitTdlibParameters" => {
                let tdlib_parameters = json!({
                    "@type": "setTdlibParameters",
                    "api_id": env::var("API_ID").expect("API_ID environment variable"),
                    "api_hash": env::var("API_HASH").expect("API_HASH environment variable"),
                    "database_directory": "example_db",
                    "application_version": "Unknown".to_string(),
                    "device_model": "Unknown".to_string(),
                    "enable_storage_optimizer": false,
                    "files_directory": None::<String>,
                    "ignore_file_names": false,
                    "system_language_code": "en",
                    "system_version": String::new(),
                    "use_chat_database": false,
                    "use_file_database": false,
                    "use_message_database": false,
                    "use_secret_chats": false,
                    "use_test_dc": false
                });
                let req = serde_json::to_string(&tdlib_parameters).unwrap();
                // dbg!(&req);

                client.send(&req);
            }
            "authorizationStateWaitPhoneNumber" => {
                let input = ask_user("Enter your phone number:");
                let req = json!({
                    "@type": "setAuthenticationPhoneNumber",
                    "phone_number": input,
                    "settings": None::<()>
                });
                client.send(&serde_json::to_string(&req).unwrap());
            }
            "authorizationStateWaitCode" => {
                let input = ask_user("Enter verification code:");
                // dbg!(&input);
                let req = json!({
                    "@type": "checkAuthenticationCode",
                    "code": input,
                });
                client.send(&serde_json::to_string(&req).unwrap());
            }
            "authorizationStateWaitPassword" => {
                let input = ask_user("Enter password:");
                let req = json!({
                    "@type": "checkAuthenticationPassword",
                    "password": input,
                });
                client.send(&serde_json::to_string(&req).unwrap());
            }
            "authorizationStateReady" => break,
            "authorizationStateClosed" => break,
            _ => {}
        }
    }

    auth_rx
}
