use crossterm::event::KeyModifiers;
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::io::{self, stdout};
use std::time::Instant;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

const ASCII_ART_2: &str = r#"
 ▄▄▄▄▄▄▄▄▄▄▄  ▄         ▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄ 
▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌
 ▀▀▀▀█░█▀▀▀▀ ▐░▌       ▐░▌▐░█▀▀▀▀▀▀▀█░▌▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀▀▀▀█░▌
     ▐░▌     ▐░▌       ▐░▌▐░▌       ▐░▌▐░▌          ▐░▌       ▐░▌
     ▐░▌     ▐░█▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄▄▄▄▄ ▐░█▄▄▄▄▄▄▄█░▌
     ▐░▌     ▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌
     ▐░▌      ▀▀▀▀█░█▀▀▀▀ ▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀█░█▀▀ 
     ▐░▌          ▐░▌     ▐░▌          ▐░▌          ▐░▌     ▐░▌  
     ▐░▌          ▐░▌     ▐░▌          ▐░█▄▄▄▄▄▄▄▄▄ ▐░▌      ▐░▌ 
     ▐░▌          ▐░▌     ▐░▌          ▐░░░░░░░░░░░▌▐░▌       ▐░▌
      ▀            ▀       ▀            ▀▀▀▀▀▀▀▀▀▀▀  ▀         ▀ 
"#;

const ASCII_ART_1: &str = r#"
 ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄       ▄▄ 
▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░▌     ▐░░▌
 ▀▀▀▀█░█▀▀▀▀ ▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀▀▀▀█░▌▐░▌░▌   ▐░▐░▌
     ▐░▌     ▐░▌          ▐░▌       ▐░▌▐░▌▐░▌ ▐░▌▐░▌
     ▐░▌     ▐░█▄▄▄▄▄▄▄▄▄ ▐░█▄▄▄▄▄▄▄█░▌▐░▌ ▐░▐░▌ ▐░▌
     ▐░▌     ▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░▌  ▐░▌  ▐░▌
     ▐░▌     ▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀█░█▀▀ ▐░▌   ▀   ▐░▌
     ▐░▌     ▐░▌          ▐░▌     ▐░▌  ▐░▌       ▐░▌
     ▐░▌     ▐░█▄▄▄▄▄▄▄▄▄ ▐░▌      ▐░▌ ▐░▌       ▐░▌
     ▐░▌     ▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░▌       ▐░▌
      ▀       ▀▀▀▀▀▀▀▀▀▀▀  ▀         ▀  ▀         ▀ 
"#;

const CORRECT: Color = Color::Green;
const INCORRECT: Color = Color::Red;

enum Keystroke {
    Wrong,
    Correct,
    Quit,
    Invalid,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let args: Vec<String> = env::args().collect();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut speed: Option<f64> = None;
    let mut races = 0.0;
    let mut total_speed = 0.0;
    let mut accuracy = 0.0;
    let default_user = String::from("default");
    let username = args.get(1).clone().unwrap_or(&default_user);

    'game: loop {
        let title = |frame: &mut Frame| {
            let areas = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Percentage(2),
                    Constraint::Percentage(35),
                    Constraint::Percentage(35),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                ],
            )
            .split(frame.size());

            frame.render_widget(
                Paragraph::new("(r) race | (q) quit").alignment(Alignment::Center),
                areas[0],
            );

            frame.render_widget(
                Paragraph::new(ASCII_ART_1)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::White)),
                areas[1],
            );
            frame.render_widget(
                Paragraph::new(ASCII_ART_2)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::White)),
                areas[2],
            );

            frame.render_widget(
                Paragraph::new(format!("{username}")).alignment(Alignment::Center),
                areas[3],
            );

            if let Some(num) = speed {
                frame.render_widget(
                    Paragraph::new(format!(
                        "Last Speed: {:.0} wpm at {:.2}% accuracy",
                        f64::round(num),
                        accuracy
                    ))
                    .alignment(Alignment::Center),
                    areas[4],
                );
            }

            if let Some(_) = speed {
                frame.render_widget(
                    Paragraph::new(format!(
                        "Session Average: {:.0} wpm",
                        f64::round(total_speed / races),
                    ))
                    .alignment(Alignment::Center),
                    areas[5],
                );
            }
        };

        terminal.draw(title)?;
        'title: loop {
            if let Ok(c) = handle_events() {
                if c == KeyCode::Char('r') {
                    break 'title;
                } else if c == KeyCode::Char('q') {
                    break 'game;
                }
            }
        }

        let mut hits = 0;
        let mut misses = 0;
        let quote = generate_quote();
        let mut position: usize = 0;
        let line = Line::from(
            quote
                .chars()
                .map(|c| Span::raw(c.to_string()))
                .collect::<Vec<Span>>(),
        );
        let mut text = Text::from(line);
        let mut start = None;
        'race: loop {
            let paragraph = Paragraph::new(text.clone());
            let mut next_char = String::new();
            if let Some(c) = quote.chars().nth(position) {
                let c = match c.to_string().to_lowercase().as_str() {
                    ":" => String::from(";"),
                    "<" => String::from(","),
                    ">" => String::from("."),
                    "?" => String::from("/"),
                    _ => c.to_string().to_lowercase(),
                };

                next_char = c;
            }

            let ui = |frame: &mut Frame| {
                let areas = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(32),
                        Constraint::Percentage(10),
                    ],
                )
                .split(frame.size());

                frame.render_widget(
                    paragraph
                        .block(Block::bordered().title("TypeRacer"))
                        .wrap(Wrap { trim: false }),
                    areas[0],
                );

                let split = Layout::new(
                    Direction::Horizontal,
                    [
                        Constraint::Percentage(32),
                        Constraint::Percentage(36),
                        Constraint::Percentage(32),
                    ],
                )
                .split(areas[1]);

                draw_keyboard(frame, &split[1], &next_char);
            };

            terminal.draw(ui)?;

            if let Some(c) = quote.chars().nth(position) {
                if let Ok(b) = handle_race(c) {
                    match b {
                        Keystroke::Correct => {
                            if start.is_none() {
                                start = Some(Instant::now());
                            }
                            if let Some(span) = text.lines[0].spans.get_mut(position) {
                                text.lines[0].spans[position] = span.clone().fg(CORRECT);
                            }
                            position += 1;
                            hits += 1;
                        }
                        Keystroke::Wrong => {
                            if start.is_none() {
                                start = Some(Instant::now());
                            }
                            if let Some(span) = text.lines[0].spans.get_mut(position) {
                                text.lines[0].spans[position] = span.clone().bg(INCORRECT);
                            }
                            misses += 1;
                        }
                        Keystroke::Quit => {
                            break 'race;
                        }
                        Keystroke::Invalid => (),
                    }
                }
            } else {
                let end: f64 = start.unwrap().elapsed().as_nanos() as f64;
                let minutes: f64 = end / 6e10 as f64;
                let words: f64 = quote.chars().count() as f64 / 5.0;
                speed = Some((words / minutes) as f64);
                total_speed += speed.unwrap();
                races += 1.0;
                accuracy = (hits * 100) as f64 / (hits + misses) as f64;
                break 'race;
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn generate_quote() -> String {
    let quotes = [
        "The quick brown fox jumps over the lazy dog.",
        "To be or not to be, that is the question.",
        "All that glitters is not gold.",
        "A journey of a thousand miles begins with a single step.",
        "To infinity and beyond!",
        "May the Force be with you.",
        "I think, therefore I am.",
        "Elementary, my dear Watson.",
        "It was the best of times, it was the worst of times.",
        "In the beginning, God created the heavens and the earth.",
        "The only thing we have to fear is fear itself.",
        "That's one small step for man, one giant leap for mankind.",
        "Float like a butterfly, sting like a bee.",
        "I'm the king of the world!",
        "Life is what happens when you're busy making other plans.",
        "You can't handle the truth!",
        "I'll be back.",
        "Keep your friends close, but your enemies closer.",
        "Houston, we have a problem.",
        "Why so serious?",
        // Add more quotes here to expand the pool
    ];

    let mut rng = rand::thread_rng();
    let random_number: usize = rng.gen_range(0..20);
    String::from(quotes[random_number])
}

fn handle_race(c: char) -> io::Result<Keystroke> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char(c) {
                return Ok(Keystroke::Correct);
            } else if key.code == KeyCode::Char('c')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                return Ok(Keystroke::Quit);
            } else if key.kind == event::KeyEventKind::Press {
                return Ok(Keystroke::Wrong);
            }
        }
    }
    return Ok(Keystroke::Invalid);
}

fn draw_keyboard(frame: &mut Frame, area: &Rect, next_c: &str) {
    let mut keyboard_layouts: HashMap<String, Vec<Vec<&str>>> = HashMap::new();

    let qwerty = vec![
        vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
        vec!["a", "s", "d", "f", "g", "h", "j", "k", "l", ";"],
        vec!["z", "x", "c", "v", "b", "n", "m", ",", ".", "/"],
    ];

    keyboard_layouts.insert("qwerty".to_string(), qwerty);

    let keyboard = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .split(*area);

    let constraints = [Constraint::Percentage(10); 10];
    let row_1 = Layout::new(Direction::Horizontal, constraints).split(keyboard[0]);
    let one = vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"];
    for (i, c) in one.iter().enumerate() {
        let mut paragraph = Paragraph::new(*c)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);

        if *c == next_c {
            paragraph = paragraph.style(Style::default().fg(CORRECT));
        }

        (*frame).render_widget(paragraph, row_1[i]);
    }

    let row_2 = Layout::new(Direction::Horizontal, constraints).split(keyboard[1]);

    let two = vec!["a", "s", "d", "f", "g", "h", "j", "k", "l", ";"];
    for (i, c) in two.iter().enumerate() {
        let mut paragraph = Paragraph::new(*c)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);

        if *c == next_c {
            paragraph = paragraph.style(Style::default().fg(CORRECT));
        }

        (*frame).render_widget(paragraph, row_2[i]);
    }

    let row_3 = Layout::new(Direction::Horizontal, constraints).split(keyboard[2]);

    let three = vec!["z", "x", "c", "v", "b", "n", "m", ",", ".", "/"];
    for (i, c) in three.iter().enumerate() {
        let mut paragraph = Paragraph::new(*c)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);

        if *c == next_c {
            paragraph = paragraph.style(Style::default().fg(CORRECT));
        }

        (*frame).render_widget(paragraph, row_3[i]);
    }

    let row_4 = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ],
    )
    .split(keyboard[3]);

    let mut spacebar = Paragraph::new("qwerty")
        .block(Block::bordered().border_type(BorderType::Rounded))
        .alignment(Alignment::Center);

    if next_c == " " {
        spacebar = spacebar.style(Style::default().fg(CORRECT));
    }

    (*frame).render_widget(spacebar, row_4[1]);
}

fn handle_events() -> io::Result<KeyCode> {
    loop {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(key.code);
                }
            }
        }
    }
}
