#![warn(clippy::all)]

mod client;
pub mod model;

pub use client::{request, FtxClient};
