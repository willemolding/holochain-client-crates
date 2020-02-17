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
//!     connect_ws, Future,
//! };
//! use url::Url;
//! 
//! fn main() {
//!     let client_url = Url::parse("ws://localhost:1234").unwrap();
//!     let client = connect_ws(&client_url).wait().unwrap(); // connect returns a future
//! 
//!     let result = client.zome_call( // calling a function also returns a future
//!         "instance_id".into(),
//!         "zome".into(),
//!         "fn_name".into(),
//!         serde_json::Value::Null,
//!     ).wait().unwrap();
//! 
//!     println!("{}", result)
//! }
//! ```
//! 
//! Example using different transport (http)
//! 
//! ```
//! use holochain_client_core::{
//!     connect_ws, Future, ConductorClient,
//! }
//! use jsonrpc_core_client::transports::http;
//! use url::Url;
//! 
//! fn main() {
//!     let client_url = Url::parse("http://localhost:1234").unwrap();
//!     let client = http::connect<ConductorClient>(&client_url).wait().unwrap(); // connect returns a future
//! 
//!     let result = client.zome_call( // calling a function also returns a future
//!         "instance_id".into(),
//!         "zome".into(),
//!         "fn_name".into(),
//!         serde_json::Value::Null,
//!     ).wait().unwrap();
//! 
//!     println!("{}", result)
//! }
//! ```
//! 
use std::prelude::v1::*;
pub use jsonrpc_core::futures::future::Future;
use jsonrpc_core_client::{transports::ws::connect, RpcError};

use url::Url;
pub fn connect_ws(url: &Url) -> impl Future<Item = ConductorClient, Error = RpcError> {
    connect::<ConductorClient>(url)
}
mod rpc_impl_conductor_rpc {
    use super::*;
    use jsonrpc_core as _jsonrpc_core;
    /// The generated client module.
    pub mod gen_client {
        use super::*;
        
        use _jsonrpc_core::serde_json::{self};
        use _jsonrpc_core::{
            Params,
        };
        use _jsonrpc_core_client::{
            RpcChannel, RpcError, RawClient,
        };
        use jsonrpc_core_client as _jsonrpc_core_client;
        /// The Client.
        pub struct Client {
            inner: RawClient,
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
            ) -> impl Future<Item = serde_json::Value, Error = RpcError> {
                let mut map = serde_json::Map::new();

                map.insert("instance_id".into(), instance_id.into());
                map.insert("zome".into(), zome.into());
                map.insert("function".into(), function.into());
                map.insert("args".into(), args.into());

                let args_map = Params::Map(map);
                self.inner.call_method("call", args_map.into())
            }
        }
        impl From<RpcChannel> for Client {
            fn from(channel: RpcChannel) -> Self {
                Client::new(channel.into())
            }
        }
    }
}
pub use self::rpc_impl_conductor_rpc::gen_client;
/// A ConductorClient is a struct that implement RpcChannel and can therefore be used by the
/// parity jsonrpc-core-client to produce a client instance that can make calls
pub use gen_client::Client as ConductorClient;
