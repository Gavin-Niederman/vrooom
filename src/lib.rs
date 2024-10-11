#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
pub(crate) mod math;

pub mod pid;