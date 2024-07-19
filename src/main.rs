use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};
use std::io;

mod game;

#[derive(Debug, Default)]
pub struct App {
    points: u8,
    exit: bool,
}
#[derive(Debug, Default)]
pub struct Card {
    name: String,
    identifier: u8,
    value: u8,
}

pub enum Screens {
    Menu,
    Game,
    GameOver,
    Win,
}

impl App {
    pub fn run(&mut self, terminal: &mut game::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_points(),
            KeyCode::Right => self.increment_points(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_points(&mut self) {
        self.points += 1;
    }

    fn decrement_points(&mut self) {
        self.points -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let current_screen = Screens::Menu;

        let parent_layout = Layout::default().margin(1).split(area);

        // let game_layout = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints(
        //         [
        //             Constraint::Length(5),
        //             Constraint::Min(5),
        //             Constraint::Length(5),
        //         ]
        //         .as_ref(),
        //     )
        //     .split(parent_layout[0]);

        let title = Title::from(" Menu ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let startGameBlock = Block::bordered().border_set(border::THICK);
        let startGameText = Text::from(vec![Line::from(vec![
            "Start Game".into(),
            "<Enter>".blue().bold(),
        ])]);

        match current_screen {
            Screens::Menu => {
                let title = Title::from(" Menu ".bold());
                let instructions =
                    Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));
                let block = Block::bordered()
                    .title(title.alignment(Alignment::Center))
                    .title(
                        instructions
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .border_set(border::THICK);
                block.render(area, buf);
            }
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = game::init()?;
    let app_result = App::default().run(&mut terminal);
    game::restore()?;
    app_result
}
