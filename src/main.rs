extern crate core;

use std::fmt::Error;

use crate::candlesticks::Api;
use crate::terminal::Terminal;

mod candlesticks;
mod utils;
mod terminal;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut term = Terminal::default().expect("Could not start default terminal");

    term.run().await;

    Ok(())
}
