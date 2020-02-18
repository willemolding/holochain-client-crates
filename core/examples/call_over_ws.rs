use holochain_client_core::{
	connect_ws, Future,
};
use serde_json::json;
use url::Url;

fn main() {
	let client_url = Url::parse("ws://localhost:3401").expect("valid URL");
	let fut = connect_ws(&client_url)
	.and_then(|client| { // connect returns a future
		client.call( // calling a function also returns a future
	        "basic-chat".into(),
	        "chat".into(),
	        "register".into(),
	        json!({"name": "Ferris", "avatar_url": ""}),
    	)
	}).map(|result| {
		println!("success: {}", result)
	}).map_err(|err| {
		println!("error: {}", err)
	});
	tokio::run(fut);
}
