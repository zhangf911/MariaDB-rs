//! Bindings and wrapper for MariaDB / MySQL for Rust.
//!
//! To use, get a Connection object with Connection::new(server, username, password, database) and
//! then call .raw_query() if you want to use SQL syntax.  Converting from &str to *const u8 is
//! done for you, so you don't need to do it yourself.  Also, most functions on COnnection will
//! return a Result, call unwrap() if you're sure it won't have errors (or don't care about
//! potential errors)
//!
//! Currently, this is a WIP.  I plan on having it so you can pass along a struct directly to and
//! from SQL, with detection to make sure that the table and struct are match.


#![allow(dead_code)]
#![warn(missing_docs)]

extern crate libc;

mod connection;
mod cbox;
mod ffi;
mod types;
mod serialize;

pub use connection::Connection;
pub use serialize::SerializeSQL;
