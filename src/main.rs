mod type_game;
mod events;

use std::error::Error;
use std::sync::mpsc;
use std::thread;
use ratatui::{
    crossterm::{
        event::{KeyCode},
    }, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, text::{Line, Span}, widgets::{Block},
    DefaultTerminal, Frame, TerminalOptions, Viewport
};
use ratatui::widgets::Paragraph;
use crate::type_game::{TypeGame, LetterState};
use crate::events::*;

fn main() -> Result<(), Box<dyn Error>> {
    let game = TypeGame::new("hello world the quick brown brown fox jumps over the lazy dog even more letters...");

    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    });

    let g_res = run_game(&mut terminal, game);

    ratatui::restore();

    g_res
    // Ok(())
}


fn run_game(
    terminal: &mut DefaultTerminal,
    mut game: TypeGame
)-> Result<(), Box<dyn Error>> {
    let mut time_left = 60;

    let (tx, rx) = mpsc::channel();
    let handle_ch = tx.clone();
    thread::spawn(move || { handle_input(handle_ch) } );
    let handle_time = tx.clone();
    thread::spawn(move ||timer(handle_time, time_left));

    let mut redraw = true;
    loop {
        if redraw {
            terminal.draw(|frame| draw(frame, &game, time_left)).expect("TODO: panic message");
        }

        match rx.recv()? {
            TypeGameEvent::KeyPress(keypress) => {
                redraw = true;
                match keypress.code {
                    KeyCode::Char(x) => {game.push(x);}
                    KeyCode::Esc => {return Ok(())}
                    KeyCode::Backspace => {game.delete();}
                    _ => {redraw = false;}
                }
            },
            TypeGameEvent::Resize => {redraw = true},
            TypeGameEvent::Timer(num) => {
                time_left = num;
                redraw = true;
                if num == 0 {return Ok(()) };
            },
        }
    }
}


fn draw(
    frame: &mut Frame,
    game: &TypeGame,
    timer: usize)
{
    let area = frame.area();
    let block = Block::new().title(Line::from(format!("Time Left: {timer}")).centered());

    let inner = block.inner(area);

    frame.render_widget(block, area);

    let horizontal = Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)]);

    let [_, span, _] = horizontal.areas(area);

    let mut letter_offset = 0;

    let mut line1_target: Vec<Span> = vec![];
    let mut line1_err: Vec<Span> = vec![];
    let mut line2_target: Vec<Span> = vec![];
    let mut line2_err: Vec<Span> = vec![];
    let mut line3_target: Vec<Span> = vec![];
    let mut line3_err: Vec<Span> = vec![];
    for (target_span, err_span) in
            [(&mut line1_target, &mut line1_err),
            (&mut line2_target, &mut line2_err),
            (&mut line3_target, &mut line3_err)] {
        while (letter_offset % span.width) != span.width - 1 {
            match game.letters.get(letter_offset as usize) {
                None => {
                    target_span.push(" ".fg(Color::Red));
                    err_span.push(" ".fg(Color::Red));
                },
                Some(LetterState::Correct(c)) => {
                    target_span.push(c.to_string().fg(Color::Green));
                    err_span.push(" ".fg(Color::Red));
                },
                Some(LetterState::Untyped(c)) => {
                    target_span.push(c.to_string().fg(Color::Gray));
                    err_span.push(" ".fg(Color::Red));
                },
                Some(LetterState::Wrong(t,c)) => {
                    if let Some(k) = t {
                        target_span.push(k.to_string().fg(Color::Red));
                    } else { target_span.push(" ".fg(Color::Red)) };
                    err_span.push(c.to_string().fg(Color::Red));
                },
            }
            letter_offset+=1;
        }
        letter_offset +=1;
    }

    let y0 = inner.y;

    frame.render_widget(Paragraph::new(Line::from(line1_target)),
                        Rect{ x: span.x, y: y0, width: span.width, height: 1, });
    frame.render_widget(Paragraph::new(Line::from(line1_err)),
                        Rect{ x: span.x, y: y0+1, width: span.width, height: 1, });
    frame.render_widget(Paragraph::new(Line::from(line2_target)),
                        Rect{ x: span.x, y: y0+2, width: span.width, height: 1, });
    frame.render_widget(Paragraph::new(Line::from(line2_err)),
                        Rect{ x: span.x, y: y0+3, width: span.width, height: 1, });
    frame.render_widget(Paragraph::new(Line::from(line3_target)),
                        Rect{ x: span.x, y: y0+4, width: span.width, height: 1, });
    frame.render_widget(Paragraph::new(Line::from(line3_err)),
                        Rect{ x: span.x, y: y0+5, width: span.width, height: 1, });
}
