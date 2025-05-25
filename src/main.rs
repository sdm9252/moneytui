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
use crate::type_game::{TypeGame, LetterState, WordState};
use crate::events::*;

const TEST_STRING_1: &str = "hello world the quick brown brown fox jumps over the lazy dog even more letters...";
const TEST_STRING_2: &str = "Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of de Finibus Bonorum et Malorum (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, Lorem ipsum dolor sit amet.., comes from a line in section 1.10.32.";
fn main() -> Result<(), Box<dyn Error>> {
    let game = TypeGame::new(TEST_STRING_2);

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

    let mut timer_started = false;
    let mut redraw = true;
    loop {
        if redraw {
            terminal.draw(|frame| draw(frame, &game, time_left)).expect("TODO: panic message");
        }

        match rx.recv()? {
            TypeGameEvent::KeyPress(keypress) => {
                redraw = true;
                if !timer_started {
                    let handle_time = tx.clone();
                    //thread::spawn(move || timer(handle_time, time_left));
                    timer_started = true;
                }
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


fn calculate_stacks(max_width: usize, game: &TypeGame)
    -> Vec<(Vec<Span>, Vec<Span>)>
{
    // first, calculate where the new lines are in the text
    let mut newline_offsets = Vec::new();
    //tells you at which row of text the cursor is
    let mut cursor_in_offset_vec = 0;

    let mut hor_offset = 0;
    for (wordnum, word) in game.words.iter().enumerate() {
        if hor_offset == 0 { //for the first word
            hor_offset += word.letters.len();
        } else if hor_offset + 1 + word.letters.len() > max_width {
            newline_offsets.push(wordnum);
            if game.cursor >= wordnum {
                cursor_in_offset_vec += 1;
                //println!("{cursor_in_offset_vec}");
            }
            hor_offset = word.letters.len();
        } else {
            hor_offset += 1 + word.letters.len()
        }
    }

    //if cursor_in_offset_vec > 0 {
      //  println!("{:?}", newline_offsets);
        //println!("{cursor_in_offset_vec}");
    //}


    //if there are numbers in newline offsets
    //the first number of newline_offsets denotes the start word of the second line
    //the last number in newline_offsets denotes the start of the last line


    let mut line1target = Vec::new();
    let mut line1error = Vec::new();
    let mut line2target = Vec::new();
    let mut line2error = Vec::new();
    let mut line3target = Vec::new();
    let mut line3error = Vec::new();

    let word_into_display = |w_offset: usize, word: &WordState,
                             target_bar_all: &mut Vec<Span>, err_bar_all: &mut Vec<Span>| {
        let mut target_bar = Vec::new();
        let mut err_bar = Vec::new();

        for letter in &word.letters {
            match letter {
                LetterState::Correct(c) => {
                    target_bar.push(c.to_string().fg(Color::Green));
                    err_bar.push(" ".fg(Color::Red));
                }
                LetterState::Wrong(t_op, w) => {
                    match t_op {
                        None => { target_bar.push(" ".fg(Color::Red)) }
                        Some(t) => { target_bar.push(t.to_string().fg(Color::Red)) }
                    }
                    err_bar.push(w.to_string().fg(Color::Red));
                }
                LetterState::Untyped(c) => {
                    target_bar.push(c.to_string().fg(Color::White));
                    err_bar.push(" ".fg(Color::Red));
                }
            }
        }
        target_bar.push(" ".fg(Color::Green));
        err_bar.push(" ".fg(Color::Red));

        //not sure if if Im doing cursor on space correct here
        if w_offset == game.cursor {
            target_bar[word.offset] = target_bar[word.offset].clone().bg(Color::Black);
        }

        target_bar_all.append(&mut target_bar);
        err_bar_all.append(&mut err_bar);
    };

    //its all contained in the three lines, or
    if newline_offsets.len() < 3 || game.cursor < newline_offsets[0] {
        //just greedily add the words in
        for (offset, word) in game.words.iter().enumerate() {
            if newline_offsets.is_empty() || offset < newline_offsets[0] {
                word_into_display (offset, word, &mut line1target, &mut line1error);
            } else if newline_offsets.len() == 1 || offset < newline_offsets[1] {
                word_into_display (offset, word, &mut line2target, &mut line2error);
            } else if newline_offsets.len() == 2 || offset < newline_offsets[2] {
                word_into_display (offset, word, &mut line3target, &mut line3error);
            }
        }
    } else if game.cursor >= newline_offsets[newline_offsets.len()-1] {
        //start enumerating through words from the third to last item in words
        // until the end
        for (offset, word) in game.words.iter().enumerate()
            .skip(newline_offsets[newline_offsets.len() - 3] - 1) {
            if offset < newline_offsets[newline_offsets.len()-2] {
                word_into_display (offset, word, &mut line1target, &mut line1error);
            } else if offset < newline_offsets[newline_offsets.len()-1] {
                word_into_display (offset, word, &mut line2target, &mut line2error);
            } else {
                word_into_display (offset, word, &mut line3target, &mut line3error);
            }
        }
    } else {
        //this is a complete mess, need to go back to drawing board

        let first_line_start = if cursor_in_offset_vec == 1 { 0 } else {
            newline_offsets[cursor_in_offset_vec - 2]
        };

        //let first_line_start = newline_offsets[cursor_in_offset_vec - 1];
        for (offset, word) in game.words.iter().enumerate()
            .skip(first_line_start) {
            if offset < newline_offsets[cursor_in_offset_vec - 1] {
                word_into_display (offset, word, &mut line1target, &mut line1error);
            } else if offset <  newline_offsets[cursor_in_offset_vec] {
                word_into_display (offset, word, &mut line2target, &mut line2error);
            } else {
                word_into_display (offset, word, &mut line3target, &mut line3error);
            }
        }


    }


    vec![(line1target, line1error), (line2target, line2error), (line3target, line3error)]
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

    if span.width < 40 {
        frame.render_widget(Paragraph::new(Line::from("Window is too skinny!")), area);
        return;
    }

    let mystacks = calculate_stacks(span.width.into(),game);

    let y0 = inner.y;

    for (num, (line_tar, line_er)) in mystacks.into_iter().enumerate() {
        let u16num:u16 = num as u16;
        frame.render_widget(Paragraph::new(Line::from(line_tar)),
                            Rect{ x: span.x, y: y0 + 2*u16num, width: span.width, height: 1, });
        frame.render_widget(Paragraph::new(Line::from(line_er)),
                            Rect{ x: span.x, y: y0 + 2*u16num+1, width: span.width, height: 1, });
    }

}
