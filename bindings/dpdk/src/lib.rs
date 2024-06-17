#![warn(rust_2018_idioms)]

mod ffi;

pub mod eal;
pub mod tx_buffer;
pub mod zeroable;
pub mod flow;

/// Reexport of crossbeam's [thread][crossbeam_utils::thread] module
///
/// This is reexported so that downstream crates don't have to manually import crossbeam and won't
/// have version conflicts
pub use crossbeam_utils::thread;

/// Reexport of [arrayvec][arrayvec] crate
///
/// This is reexported so that downstream crates don't have to manually import arrayvec and won't
/// have version conflicts
pub use arrayvec;
