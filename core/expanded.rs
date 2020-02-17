#![feature(prelude_import)]
//!
//! Implements ConductorClient, a struct that implement RpcChannel and can therefore be used by the
//! parity jsonrpc-core-client to produce a client instance that can make calls over any transport.
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
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
pub use jsonrpc_core::futures::future::Future;
use jsonrpc_core_client::{transports::ws::connect, RpcError};
use jsonrpc_derive::rpc;
use url::Url;
pub fn connect_ws(url: &Url) -> impl Future<Item = ConductorClient, Error = RpcError> {
    connect::<ConductorClient>(url)
}
mod rpc_impl_ConductorRpc {
    use super::*;
    use jsonrpc_core as _jsonrpc_core;
    /// The generated client module.
    pub mod gen_client {
        use super::*;
        use _jsonrpc_core::futures::prelude::*;
        use _jsonrpc_core::futures::sync::{mpsc, oneshot};
        use _jsonrpc_core::serde_json::{self, Value};
        use _jsonrpc_core::{
            Call, Error, ErrorCode, Id, MethodCall, Params, Request, Response, Version,
        };
        use _jsonrpc_core_client::{
            RpcChannel, RpcError, RpcFuture, TypedClient, TypedSubscriptionStream,
        };
        use jsonrpc_core_client as _jsonrpc_core_client;
        /// The Client.
        pub struct Client {
            inner: TypedClient,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Client {
            #[inline]
            fn clone(&self) -> Client {
                match *self {
                    Client {
                        inner: ref __self_0_0,
                    } => Client {
                        inner: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl Client {
            /// Creates a new `Client`.
            pub fn new(sender: RpcChannel) -> Self {
                Client {
                    inner: sender.into(),
                }
            }
            pub fn call(
                &self,
                instance_id: String,
                zome: String,
                function: String,
                args: serde_json::Value,
            ) -> impl Future<Item = String, Error = RpcError> {
                let args_tuple = (instance_id, zome, function, args);
                self.inner.call_method("call", "String", args_tuple)
            }
        }
        impl From<RpcChannel> for Client {
            fn from(channel: RpcChannel) -> Self {
                Client::new(channel.into())
            }
        }
    }
}
pub use self::rpc_impl_ConductorRpc::gen_client;
/// A ConductorClient is a struct that implement RpcChannel and can therefore be used by the
/// parity jsonrpc-core-client to produce a client instance that can make calls
pub use gen_client::Client as ConductorClient;
