use std::io;
use std::io::{Error, stdout};
use std::task::Poll;

use serde_json::from_str;
use termion;
use termion::cursor::Restore;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::candlesticks::{Api, CandleStick, Request};
use crate::utils::{pad, seconds_to_date};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn from(width: u16, height: u16) -> Self {
        Size { width, height }
    }
}

pub struct Terminal {
    size: Size,
    info: Api,
    _stdout: RawTerminal<io::Stdout>,
    quit: bool,
    update: bool,
}

impl Terminal {
    pub fn default() -> Result<Terminal, Error> {
        let (width, height) = termion::terminal_size()?;

        Ok(Terminal {
            size: Size::from(width, height),
            info: Api::mexc(),
            _stdout: stdout().into_raw_mode()?,
            quit: false,
            update: true,
        })
    }

    pub async fn run(&mut self) {
        // Start off by clearing screen and hide the cursor
        println!("{}", termion::cursor::Hide);
        println!("{}", termion::clear::All);

        loop {
            if self.quit {
                break;
            }

            if self.update {
                self.fetching_data_screen();

                let req = self.make_request().await;

                match &req {
                    None => {}
                    Some(req) => {
                        self.display_first_lines();
                        self.display_api_data(req);
                        self.last_line();
                        self.display_commands();
                    }
                }

                self.update = false;
            }

            let key = self.await_keypress();
            match key {
                Ok(key) => self.handle_keypress(key),
                Err(_) => {}
            }
        }
    }

    fn handle_keypress(&mut self, key: Key) {
        match key {
            Key::Ctrl('q') => { self.quit = true; }
            Key::Ctrl('u') => { self.update = true; }
            _ => {}
        }
    }

    fn await_keypress(&self) -> Result<Key, Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    fn display_api_data(&self, req: &Request) {
        for i in 0..self.size.height - 8 {
            let line: String = match req.data.get(i as usize) {
                Some(candlestick) => format!("{}", self.candlestick_line(candlestick)),
                None => String::default()
            };

            println!("{}\r", line);
        }
    }

    fn candlestick_line(&self, cs: &CandleStick) -> String {
        // The -2 account for the |'s with which the line must start
        let line_length = self.size.width - 2;
        let space = ((self.size.width - 3) / 5 - 1) as usize;

        format!(" │{}│{}│{}│{}│{}│",
                pad(&*seconds_to_date(cs.timestamp), space, " "),
                pad(&*cs.open.to_string(), space, " "),
                pad(&*cs.close.to_string(), space, " "),
                pad(&*cs.high.to_string(), space, " "),
                pad(&*cs.low.to_string(), space, " "),
        )
    }

    fn display_first_lines(&self) {
        let message = format!("{} -- {}", self.info.symbol, self.info.interval);
        let padding = (self.size.width as usize - message.len()) / 2;

        // The padding will center the message
        println!("{}{}\r\n\r", " ".repeat(padding as usize), message);

        // Now show the column headers
        let space = ((self.size.width - 3) / 5 - 1) as usize;
        println!(" ┌{}┬{}┬{}┬{}┬{}┐\r",
                 pad(" Date ", space, "─"),
                 pad(" Open ", space, "─"),
                 pad(" Close ", space, "─"),
                 pad(" High ", space, "─"),
                 pad(" Low ", space, "─"),
        );
    }

    fn last_line(&self) {
        // The -2 account for the |'s with which the line must start
        let line_length = self.size.width - 2;
        let space = ((self.size.width - 3) / 5 - 1) as usize;

        println!(" └{}┴{}┴{}┴{}┴{}┘\r",
                 pad("", space, "─"),
                 pad("", space, "─"),
                 pad("", space, "─"),
                 pad("", space, "─"),
                 pad("", space, "─"),
        )
    }

    fn display_commands(&self) {
        println!("  Commands: \n\r  \tQuit: Ctr+Q \t Update data: Ctr+U \r");
    }

    fn fetching_data_screen(&self) {
        println!("{}", termion::cursor::Hide);
        println!("{}", termion::clear::All);

        let message = "Fetching data...";

        for i in 0..self.size.height {
            if i == self.size.height / 3 {
                println!("{}{}\r", " ".repeat(((self.size.width) as usize - message.len()) / 2 as usize), message);
            } else {
                println!("\r");
            }
        }
    }

    async fn make_request(&self) -> Option<Request> {
        Some(self.info.make_request().await.ok()?)
    }
}

