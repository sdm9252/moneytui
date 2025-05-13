use std::error::Error;
use ratatui::{
    backend::Backend, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    }, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, symbols, text::{Line, Span}, widgets::{Block, Gauge, LineGauge, List, ListItem, Paragraph, Widget}, DefaultTerminal, Frame, Terminal, TerminalOptions, Viewport
};

pub struct TypeGame {
    pub target_text: String,
    pub player_input: String,
}

impl TypeGame {
    fn new() -> TypeGame {
        TypeGame {
            target_text: String::from("the quick brown fox jumps over the lazy dog"),
            player_input: String::new(),
        }
    }

    fn push(&mut self, input: char) {
        self.player_input.push(input);
    }

    fn delete(&mut self) {
        self.player_input.pop();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = TypeGame::new();

    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(8),
    });

    let g_res = run_game(&mut terminal, game);

    ratatui::restore();

    Ok(())
}

fn run_game(
    terminal: &mut DefaultTerminal,
    mut game: TypeGame
)-> Result<(), Box<dyn Error>> {

    let mut redraw = true;
    loop {
        if redraw {
            terminal.draw(|frame| draw(frame, &game));
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            if key.code == KeyCode::Esc {
                return  Ok(());
            }
        }
    }
    Ok(())
}


fn draw(frame: &mut Frame, game: &TypeGame) {
    let area = frame.area();
    let block = Paragraph::new(game.target_text.clone()).fg(Color::Red);
    frame.render_widget(block, area);
}
