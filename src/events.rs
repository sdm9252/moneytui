use std::sync::mpsc;
use std::time::Duration;
use crossterm::event::{Event, KeyEvent};
use ratatui::crossterm::event;
use crate::events::TypeGameEvent::{KeyPress, Resize, Timer};

#[derive(Debug)]
pub enum TypeGameEvent {
    KeyPress (KeyEvent),
    Resize,
    Timer(usize),
}

pub fn handle_input(tx: mpsc::Sender<TypeGameEvent>) {
    loop {
        match event::read().unwrap() {
            Event::Key(key_event) => {tx.send(KeyPress(key_event)).unwrap();}
            Event::Resize(_, _) => {tx.send(Resize).unwrap();}
            _ => {}
        }
    }
}

pub fn timer(tx: mpsc::Sender<TypeGameEvent>, seconds: usize) {
    let mut countdown = seconds;
    loop {
        tx.send(Timer(countdown)).unwrap();
        if countdown == 0 {return;}
        countdown -= 1;
        std::thread::sleep(Duration::from_secs(1));
    }
}