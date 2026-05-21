//! This is a library for working with anything related to actual data on
//! the Minerva System. This includes:
//!
//! - Data Transfer Objects (DTOs) for any defined entities;
//! - Conversions from gRPC messages to actual entity DTOs;
//! - Loading and reading project configuration such as multi-tenant config;
//! - Logging to screen (non-audit logging).

#![warn(clippy::all)]
#![warn(missing_docs)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate num_derive;

#[allow(missing_docs)]
#[allow(unused_imports)]
#[rustfmt::skip]
pub mod schema;

pub mod db;
pub mod encryption;
pub mod file;
pub mod log;
pub mod mongo;
pub mod session;
#[allow(missing_docs)]
pub mod syslog;
pub mod tenancy;
pub mod user;
