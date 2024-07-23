use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding, Paragraph, Widget,
    },
    Frame,
};
use std::io;

mod game;

const CARD_WIDTH: u16 = 9;
const CARD_HEIGHT: u16 = 6;

#[derive(Debug)]
enum CardsValues {
    As,
    Dos,
    Tres,
    Cuatro,
    Cinco,
    Seis,
    Siete,
    Ocho,
    Nueve,
    Diez,
    Sota,
    Caballo,
    Rey,
}
#[derive(Debug)]
enum Palos {
    Espadas,
    Bastos,
    Copas,
    Oros,
}

fn get_card_value(card: &Card) -> u8 {
    match card.value {
        CardsValues::As => 11,
        CardsValues::Tres => 10,
        CardsValues::Rey => 4,
        CardsValues::Caballo => 3,
        CardsValues::Sota => 2,
        _ => 0,
    }
}

fn get_card_name(card: &Card) -> &str {
    match card.value {
        CardsValues::As => "As",
        CardsValues::Dos => "Dos",
        CardsValues::Tres => "Tres",
        CardsValues::Cuatro => "Cuatro",
        CardsValues::Cinco => "Cinco",
        CardsValues::Seis => "Seis",
        CardsValues::Siete => "Siete",
        CardsValues::Ocho => "Ocho",
        CardsValues::Nueve => "Nueve",
        CardsValues::Diez => "Diez",
        CardsValues::Sota => "Sota",
        CardsValues::Caballo => "Caballo",
        CardsValues::Rey => "Rey",
    }
}

fn get_card_emoji(card: &Card) -> char {
    match card.palo {
        Palos::Espadas => '⚔',
        Palos::Bastos => '🏏',
        Palos::Copas => '🏆',
        Palos::Oros => '🪙',
    }
}

#[derive(Debug, Default)]
pub struct App {
    points: u8,
    exit: bool,
    current_screen: Screens,
    opponent_cards: Vec<Card>,
    player_cards: Vec<Card>,
    selected_card: Option<u8>,
}
#[derive(Debug)]
pub struct Card {
    value: CardsValues,
    palo: Palos,
}

#[derive(Debug)]
pub enum Screens {
    Menu,
    Game,
    GameOver,
    Win,
}

impl Default for Screens {
    fn default() -> Self {
        Screens::Menu
    }
}

impl App {
    pub fn new() -> Self {
        App {
            points: 0,
            exit: false,
            current_screen: Screens::Menu,
            opponent_cards: vec![
                Card {
                    value: CardsValues::As,
                    palo: Palos::Bastos,
                },
                Card {
                    value: CardsValues::Dos,
                    palo: Palos::Bastos,
                },
                Card {
                    value: CardsValues::Tres,
                    palo: Palos::Espadas,
                },
                Card {
                    value: CardsValues::Cuatro,
                    palo: Palos::Oros,
                },
                Card {
                    value: CardsValues::Sota,
                    palo: Palos::Copas,
                },
            ],
            player_cards: vec![
                Card {
                    value: CardsValues::As,
                    palo: Palos::Copas,
                },
                Card {
                    value: CardsValues::Dos,
                    palo: Palos::Copas,
                },
                Card {
                    value: CardsValues::Tres,
                    palo: Palos::Copas,
                },
                Card {
                    value: CardsValues::Cuatro,
                    palo: Palos::Copas,
                },
            ],
            selected_card: Some(3),
        }
    }
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
            KeyCode::Enter => self.set_screen(Screens::Game),
            _ => {}
        }
    }

    fn set_screen(&mut self, screen: Screens) {
        self.current_screen = screen;
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
    fn select_card(&mut self, card: u8) {
        self.selected_card = Some(card);
    }
    fn get_selected_card(&self) -> Option<&Card> {
        let c = self.selected_card.unwrap();
        self.selected_card
            .map(|card| &self.player_cards[card as usize])
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let current_screen = &self.current_screen;

        let parent_layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(0)
            .split(area);

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
                Paragraph::new(vec![
                    Line::from("Start new Game"),
                    Line::from("<Enter>").blue().bold(),
                ])
                .alignment(Alignment::Center)
                .block(block)
                .render(parent_layout[0], buf);
            }
            Screens::Game => {
                //DEFINE MAIN GAME LAYOUT
                let game_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(10),
                            Constraint::Min(5),
                            Constraint::Length(10),
                        ]
                        .as_ref(),
                    )
                    .split(parent_layout[0]);
                let block = Block::bordered().border_set(border::PLAIN);

                let title = Title::from(" Game ".bold());
                let instructions =
                    Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));
                let parent_block = Block::bordered()
                    .title(title.alignment(Alignment::Center))
                    .title(
                        instructions
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .border_set(border::THICK);

                let top_game_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
                    .split(game_layout[0]);

                //RENDER CARDS OF THE TOP
                let top_game_cards_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints::<&Vec<Constraint>>(
                        (0..self.opponent_cards.len())
                            .map(|_| Constraint::Percentage(100 / self.opponent_cards.len() as u16))
                            .collect::<Vec<Constraint>>()
                            .as_ref(),
                    )
                    .split(top_game_layout[1]);

                let card_block = Block::default().on_red();
                for (i, card) in self.opponent_cards.iter().enumerate() {
                    //let card_area = centered_rect(40, 60, top_game_cards_layout[i]);
                    let card_area = center(
                        top_game_cards_layout[i],
                        Constraint::Length(CARD_WIDTH),
                        Constraint::Length(CARD_HEIGHT),
                    );
                    let card_text = Text::from(vec![
                        Line::from(get_card_name(&card).to_string()),
                        Line::from(get_card_emoji(&card).to_string()),
                    ]);
                    Paragraph::new(card_text)
                        .alignment(Alignment::Center)
                        .block(card_block.clone())
                        .render(card_area, buf);
                }

                //RENDER POINTS AND OPPONENT CARDS
                Paragraph::new(vec![
                    Line::from("Puntuation"),
                    Line::from(self.points.to_string()),
                ])
                .alignment(Alignment::Left)
                .block(block.clone())
                .render(top_game_layout[0], buf);
                Paragraph::new("Opponent Cards")
                    .alignment(Alignment::Center)
                    .block(block.clone())
                    .render(top_game_layout[1], buf);

                //RENDER PLAYER CARDS

                let player_cards_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints::<&Vec<Constraint>>(
                        (0..self.player_cards.len())
                            .map(|_| Constraint::Percentage(100 / self.player_cards.len() as u16))
                            .collect::<Vec<Constraint>>()
                            .as_ref(),
                    )
                    .split(game_layout[2]);

                for (i, card) in self.player_cards.iter().enumerate() {
                    let card_area = center(
                        player_cards_layout[i],
                        Constraint::Length(CARD_WIDTH),
                        Constraint::Length(CARD_HEIGHT),
                    );
                    let card_text = Text::from(vec![
                        Line::from(get_card_name(&card).to_string()),
                        Line::from(get_card_emoji(&card).to_string()),
                    ]);

                    let mut user_card_block = Block::default().on_green().title(
                        Title::from((i + 1).to_string())
                            .position(Position::Bottom)
                            .alignment(Alignment::Center),
                    );
                    if let Some(slctd) = self.selected_card {
                        let i = i as u8;
                        if slctd == i {
                            user_card_block = Block::bordered()
                                .on_green()
                                .title(
                                    Title::from((i + 1).to_string())
                                        .position(Position::Bottom)
                                        .alignment(Alignment::Center),
                                )
                                .border_set(border::DOUBLE);
                        }
                    }

                    Paragraph::new(card_text)
                        .alignment(Alignment::Center)
                        .block(user_card_block)
                        .render(card_area, buf);
                }

                Paragraph::new("Table where the game is being played")
                    .alignment(Alignment::Center)
                    .block(block.clone())
                    .render(game_layout[1], buf);
                Paragraph::new("Your Cards")
                    .alignment(Alignment::Center)
                    .block(block.clone())
                    .render(game_layout[2], buf);
                parent_block.render(parent_layout[0], buf)
            }
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = game::init()?;
    let app_result = App::new().run(&mut terminal);
    game::restore()?;
    app_result
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
