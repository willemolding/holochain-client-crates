//!
//! Implements ConductorClient, a struct that implement RpcChannel and can therefore be used by the 
//! parity jsonrpc-core-client to produce a client instance that can make calls over any transport.
//! 
//! WARNING! - A limitation of the client macro in jsonrpc-core-client is preventing this from working properly
//! The expanded and fixed macro is currently in the lib.rs. Once the problem is fixed this will become the lib and 
//! the macro exansion can work as intended.
//! 
//! It also exports a connect_ws function which is a helper to construct websocket connection
//! 
//! Basic Example
//! ```
//! use holochain_client_core::{
//! 	connect_ws, Future,
//! };
//! use url::Url;
//! 
//! fn main() {
//! 	let client_url = Url::parse("ws://localhost:1234").unwrap();
//! 	let client = connect_ws(&client_url).wait().unwrap(); // connect returns a future
//! 
//!     let result = client.zome_call( // calling a function also returns a future
//!         "instance_id".into(),
//!         "zome".into(),
//!         "fn_name".into(),
//!         serde_json::Value::Null,
//!     ).wait().unwrap();
//! 
//! 	println!("{}", result)
//! }
//! ```
//! 
//! Example using different transport (http)
//! 
//! ```
//! use holochain_client_core::{
//! 	connect_ws, Future, ConductorClient,
//! }
//! use jsonrpc_core_client::transports::http;
//! use url::Url;
//! 
//! fn main() {
//! 	let client_url = Url::parse("http://localhost:1234").unwrap();
//! 	let client = http::connect<ConductorClient>(&client_url).wait().unwrap(); // connect returns a future
//! 
//!     let result = client.zome_call( // calling a function also returns a future
//!         "instance_id".into(),
//!         "zome".into(),
//!         "fn_name".into(),
//!         serde_json::Value::Null,
//!     ).wait().unwrap();
//! 
//! 	println!("{}", result)
//! }
//! ```
//! 
use jsonrpc_derive::rpc;
use url::Url;
use jsonrpc_core_client::{
	RpcError,
	transports::ws::connect,
};
pub use jsonrpc_core::futures::future::Future;

pub fn connect_ws(url: &Url) -> impl Future<Item = ConductorClient, Error = RpcError> {
	connect::<ConductorClient>(url)
}

#[rpc(client)]
pub trait ConductorRpc {
    #[rpc(name = "call")]    
    fn call(
        &self,
        instance_id: String,
        zome: String,
        function: String,
        args: serde_json::Value,
    ) -> Result<String>;

    // TODO: Add other conductor calls (e.g. admin calls, add_instance etc)
}

/// A ConductorClient is a struct that implement RpcChannel and can therefore be used by the 
/// parity jsonrpc-core-client to produce a client instance that can make calls
pub use gen_client::Client as ConductorClient;
