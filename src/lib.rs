#[macro_use]
extern crate derive_builder;

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate bytes;
extern crate rand;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate tokio;

extern crate futures;
extern crate tokio_codec;
extern crate tokio_executor;
extern crate tokio_io;
extern crate tokio_tcp;
extern crate tokio_timer;

#[macro_use]
mod error;

// TODO: Add tokio-codec traits impl
// TODO: Manage connection to server via tcp
// TODO: Add SSL connection to server
// TODO: dunno lol

pub use self::error::*;
pub mod codec;
pub mod protocol;