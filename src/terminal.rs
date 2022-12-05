use std::io;
use std::io::{Error, stdout};
use std::task::Poll;

use termion;
use termion::cursor::Restore;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::candlesticks::{Api, CandleStick, Request};

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
}

impl Terminal {
    pub fn default() -> Result<Terminal, Error> {
        let (width, height) = termion::terminal_size()?;

        Ok(Terminal {
            size: Size::from(width, height),
            info: Api::mexc(),
            _stdout: stdout().into_raw_mode()?,
            quit: false,
        })
    }

    pub async fn run(&mut self) {
        // Start off by clearing screen and hide the cursor
        println!("{}", termion::cursor::Hide);
        println!("{}", termion::clear::All);

        let req = self.make_request().await;

        loop {
            if self.quit {
                break;
            }

            match &req {
                None => {}
                Some(req) => {
                    self.display_api_data(req)
                }
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
        for i in 0..self.size.height - 5 {
            let line: String = match req.data.get(i as usize) {
                Some(candlestick) => candlestick.to_str(),
                None => String::default()
            };

            println!("{}\r", line);
        }
    }

    async fn make_request(&self) -> Option<Request> {
        Some(self.info.make_request().await.ok()?)
    }
}

