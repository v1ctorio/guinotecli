use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Rectangle},
        Block, Borders, Padding, Paragraph, Widget,
    },
    Frame,
};
use std::io;

mod game;

const MIN_TERMINAL_WIDTH: u16 = 35;
const MIN_TERMINAL_HEIGHT: u16 = 140;

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
    Sota,
    Caballo,
    Rey,
}
#[derive(Debug, PartialEq)]
enum Palos {
    Espadas,
    Bastos,
    Copas,
    Oros,
}

impl Card {
    fn emoji(&self) -> char {
        match self.palo {
            Palos::Espadas => '⚔',
            Palos::Bastos => '🏏',
            Palos::Copas => '🏆',
            Palos::Oros => '🪙',
        }
    }
    fn name(&self) -> &str {
        match self.value {
            CardsValues::As => "As",
            CardsValues::Dos => "Dos",
            CardsValues::Tres => "Tres",
            CardsValues::Cuatro => "Cuatro",
            CardsValues::Cinco => "Cinco",
            CardsValues::Seis => "Seis",
            CardsValues::Siete => "Siete",
            CardsValues::Sota => "Sota",
            CardsValues::Caballo => "Caballo",
            CardsValues::Rey => "Rey",
        }
    }

    fn value(&self) -> u8 {
        match self.value {
            CardsValues::As => 11,
            CardsValues::Tres => 10,
            CardsValues::Rey => 4,
            CardsValues::Caballo => 2,
            CardsValues::Sota => 3,
            _ => 0,
        }
    }

    fn kill_power(&self) -> u8 {
        // In guiñote the power of a card defeating another is not the same as the points value of the card
        match self.value {
            CardsValues::As => 12,
            CardsValues::Tres => 11,
            CardsValues::Rey => 10,
            CardsValues::Sota => 9,
            CardsValues::Caballo => 8,
            CardsValues::Siete => 7,
            CardsValues::Seis => 6,
            CardsValues::Cinco => 5,
            CardsValues::Cuatro => 4,
            CardsValues::Dos => 3,
        }
    }
}

impl Palos {
    fn to_string(&self) -> String {
        match self {
            Palos::Espadas => "⚔ Espadas",
            Palos::Bastos => "🏏 Bastos",
            Palos::Copas => "🏆 Copas",
            Palos::Oros => "🪙 Oros",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct App {
    points: u8,
    opponent_points: u8,
    exit: bool,
    current_screen: Screens,
    opponent_cards: Vec<Card>,
    player_cards: Vec<Card>,
    selected_card: Option<u8>,
    triunfo: Palos,
    last_played_card: Option<Card>,
    last_played_opponent_card: Option<Card>,
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
    ResolutionError,
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
            opponent_points: 0,
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
            last_played_card: Some(Card {
                value: CardsValues::Cuatro,
                palo: Palos::Bastos,
            }),
            last_played_opponent_card: None,
            triunfo: Palos::Copas,
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
            Event::Resize(height, width) => {
                if height < MIN_TERMINAL_HEIGHT || width < MIN_TERMINAL_WIDTH {
                    self.set_screen(Screens::ResolutionError);
                }
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
            KeyCode::Char('1') => self.select_card(0),
            KeyCode::Char('2') => self.select_card(1),
            KeyCode::Char('3') => self.select_card(2),
            KeyCode::Char('4') => self.select_card(3),
            //DEBUG
            KeyCode::Char('5') => self.set_screen(Screens::GameOver),
            KeyCode::Char('6') => self.set_screen(Screens::Win),
            _ => {}
        }
    }

    fn add_points(&mut self, cards: Vec<Card>, target: u8) {
        //if 0 add points to player, if 1 add points to opponent
        let points_to_add: u8 = cards
            .iter()
            .map(|card| match card.value {
                CardsValues::Sota => 3,
                CardsValues::Caballo => 2,
                CardsValues::Rey => 4,
                CardsValues::As => 11,
                CardsValues::Tres => 10,
                _ => 0,
            })
            .sum();
        if target == 0 {
            self.points += points_to_add;
        } else {
            self.opponent_points += points_to_add;
        }
    }

    fn do_x_defeat_y(&self, own: Card, opponent: Card) -> bool {
        let triunfo = &self.triunfo;
        if (own.palo == triunfo && !matches!(opponent.palo, triunfo)) {
            return true;
        }
        if matches!(own.palo, opponent.palo) {
            return own.kill_power() > opponent.kill_power();
        }
        if own.palo != triunfo && opponent.palo == triunfo {
            return false;
        }
        own.kill_power() > opponent.kill_power()
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
        let crown = r#"      <>
        .::::.
        @\\/W\/\/W\//@
        \\/^\/\/^\//
        \_O_<>_O_/
        "#
        .to_string();

        let current_screen = &self.current_screen;

        let parent_layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(0)
            .split(area);

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
                        Line::from(card.name().to_string()),
                        Line::from(card.emoji().to_string()),
                    ]);
                    Paragraph::new(card_text)
                        .alignment(Alignment::Center)
                        .block(card_block.clone())
                        .render(card_area, buf);
                }

                //RENDER POINTS AND OPPONENT CARDS

                let vertically_divided_info_box = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(10),
                        Constraint::Percentage(45),
                        Constraint::Percentage(45),
                    ])
                    .split(top_game_layout[0]);

                let vertically_divided_top_part_layout_wo_margin = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
                    .split(vertically_divided_info_box[1]);

                let horizontally_divided_info_box = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(vertically_divided_top_part_layout_wo_margin[1]);

                Paragraph::new(vec![
                    Line::from("Yours"),
                    Line::from(self.points.to_string()).alignment(Alignment::Center),
                ])
                .alignment(Alignment::Right)
                .block(block.clone())
                .render(horizontally_divided_info_box[0], buf);

                Paragraph::new(vec![
                    Line::from("Opponent"),
                    Line::from(self.points.to_string()).alignment(Alignment::Center),
                ])
                .alignment(Alignment::Left)
                .block(block.clone())
                .render(horizontally_divided_info_box[1], buf);

                Block::bordered()
                    .border_set(border::ROUNDED)
                    .title(Title::from("Points").alignment(Alignment::Center))
                    .on_blue()
                    .render(vertically_divided_top_part_layout_wo_margin[1], buf);

                Paragraph::new(vec![
                    Line::from("Triunfo").alignment(Alignment::Center),
                    Line::from(self.triunfo.to_string()).alignment(Alignment::Center),
                ])
                .render(
                    center(
                        vertically_divided_info_box[2],
                        Constraint::Percentage(100),
                        Constraint::Percentage(100),
                    ),
                    buf,
                );

                //OPPONENT CARDS
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
                    let card_canvas = Canvas::default().paint(|ctx| {
                        ctx.draw(&Rectangle {
                            x: 0.0,
                            y: 0.0,
                            width: CARD_WIDTH as f64,
                            height: CARD_HEIGHT as f64,
                            color: Color::Green,
                        });
                    });

                    let card_area = center(
                        player_cards_layout[i],
                        Constraint::Length(CARD_WIDTH),
                        Constraint::Length(CARD_HEIGHT),
                    );
                    let card_text = Text::from(vec![
                        Line::from(card.name()),
                        Line::from(card.emoji().to_string()),
                    ]);

                    let mut user_card_block = Block::bordered()
                        .on_green()
                        .border_set(border::ONE_EIGHTH_TALL)
                        .title(
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
                    card_canvas.render(card_area, buf);
                }

                Paragraph::new("Table where the game is being played")
                    .alignment(Alignment::Center)
                    .block(block.clone())
                    .render(game_layout[1], buf);
                Paragraph::new("Your Cards")
                    .alignment(Alignment::Center)
                    .block(block.clone())
                    .render(game_layout[2], buf);
                parent_block.render(parent_layout[0], buf);

                //RENDER TABLE LIKE MIDDLE PART

                let layout_middle_vertically_divided_opponent_cards = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(20),
                        Constraint::Percentage(50),
                    ])
                    .split(game_layout[1]);
                let constraint_for_opponent_card =
                    layout_middle_vertically_divided_opponent_cards[1];

                if let Some(lst) = &self.last_played_opponent_card {
                    let card_area = center(
                        constraint_for_opponent_card,
                        Constraint::Length(CARD_WIDTH),
                        Constraint::Length(CARD_HEIGHT),
                    );
                    let crd_blck = Block::bordered()
                        .border_set(border::PROPORTIONAL_TALL)
                        .on_red();

                    Paragraph::new(Line::from(lst.name()))
                        .block(crd_blck)
                        .render(card_area, buf)
                }

                //render own cards
                let layout_own_card_on_board = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(20),
                        Constraint::Percentage(30),
                    ])
                    .split(game_layout[1]);
                let constraint_for_card_board = layout_own_card_on_board[1];

                if let Some(last) = &self.last_played_card {
                    let card_area = center(
                        constraint_for_card_board,
                        Constraint::Length(CARD_WIDTH),
                        Constraint::Length(CARD_HEIGHT),
                    );
                    let crd_blck = Block::bordered()
                        .border_set(border::PROPORTIONAL_TALL)
                        .on_red();

                    Paragraph::new(Line::from(last.name()))
                        .block(crd_blck)
                        .render(card_area, buf)
                }
            }
            Screens::ResolutionError => {
                let text = vec![
                    Line::from("The terminal size is too small to play the game"),
                    Line::from(format!(
                        "Please resize the terminal to at least {}x{}",
                        MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT
                    )),
                ];
                let text = Text::from(text);
                Paragraph::new(text)
                    .alignment(Alignment::Center)
                    .block(Block::bordered().border_set(border::FULL))
                    .render(area, buf);
            }
            Screens::Win => {
                let text = vec![
                    Line::from("Congratulations! You have won the game"),
                    Line::from("Press 'q' to quit the game"),
                ];
                let text = Text::from(text);
                Paragraph::new(text)
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL))
                    .render(area, buf);
            }
            Screens::GameOver => {
                let area = center(area, Constraint::Percentage(50), Constraint::Percentage(50));
                let text = vec![
                    Line::from("You have lost the game! You better practive more!"),
                    Line::from("Press 'q' to quit the game"),
                    Line::styled(
                        ":(((((((((((((((((((((((((((((((((",
                        Style::new()
                            .fg(Color::Red)
                            .add_modifier(Modifier::RAPID_BLINK),
                    ),
                ];
                let text = Text::from(text);
                Paragraph::new(text)
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .on_magenta()
                            .padding(Padding::uniform(4)),
                    )
                    .render(area, buf);
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
