mod app;
mod cmd;
mod common;
mod handle;
#[cfg(test)]
mod integration_test;
mod model;
mod msg;
mod update;
mod view;

pub use app::run;
