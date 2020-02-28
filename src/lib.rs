#![feature(or_patterns)]
#![feature(bool_to_option)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate async_trait;

pub mod aggregator;
pub mod common;
pub mod coordinator;
pub mod metric_store;
