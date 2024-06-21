mod types;
mod ui;
mod utils;

use crossterm::event::KeyModifiers;
use std::env;
use std::io::{self, stdout};
use std::time::Instant;
use types::{Keystroke, Race, Split, CORRECT, INCORRECT, TITLE};
use utils::{get_keyboard_layout, get_quote};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let args: Vec<String> = env::args().collect();

    let mut total_words = 0.0;
    let mut total_time = 0.0;
    let mut last_race: Option<Race> = None;
    let default_user = String::from("default");
    let username = args.get(1).clone().unwrap_or(&default_user);
    let layout = args.get(2).clone();

    'game: loop {
        let mut show_results = true;
        'title: loop {
            terminal.draw(|frame| {
                ui::title(
                    frame,
                    last_race.clone(),
                    total_words,
                    total_time,
                    username,
                    show_results,
                )
            })?;
            if let Ok(c) = handle_events() {
                if c == KeyCode::Char('s') {
                    break 'title;
                } else if c == KeyCode::Char('q') {
                    break 'game;
                } else if c == KeyCode::Char('r') {
                    show_results = if show_results { false } else { true };
                }
            }
        }
        let mut hits = 0;
        let mut misses = 0;
        let mut set: usize = 0;

        let quote = get_quote();
        let (quote_name, quote_text) = (quote.get_name(), quote.get_text());

        let mut position: usize = 0;
        let line = Line::from(
            quote_text
                .chars()
                .map(|c| Span::raw(c.to_string()))
                .collect::<Vec<Span>>(),
        );
        let mut text = Text::from(line);

        let mut start: Option<Instant> = None;
        let mut interval = 0;
        let mut splits: Vec<Split> = Vec::new();

        'race: loop {
            if let Some(time) = start {
                let elapsed = time.elapsed().as_nanos();
                if elapsed - interval >= 1e9 as u128 {
                    splits.push(Split::new(hits, misses, elapsed));
                    interval = elapsed;
                }
            }

            let paragraph = Paragraph::new(text.clone());
            let mut next_char = String::new();

            if let Some(c) = quote_text.chars().nth(position) {
                let c = match c.to_string().to_lowercase().as_str() {
                    ":" => String::from(";"),
                    "<" => String::from(","),
                    ">" => String::from("."),
                    "?" => String::from("/"),
                    _ => c.to_string(),
                };

                next_char = c.to_lowercase();
            }

            terminal.draw(|frame| {
                ui::race(
                    frame,
                    quote_name.clone(),
                    paragraph.clone(),
                    layout,
                    next_char.clone(),
                )
            })?;

            if let Some(c) = quote_text.chars().nth(position) {
                if position == set {
                    if let Some(span) = text.lines[0].spans.get_mut(position) {
                        text.lines[0].spans[position] = span.clone().bg(TITLE);
                        set += 1;
                    }
                }

                if let Ok(b) = handle_race(c) {
                    match b {
                        Keystroke::Correct => {
                            if start.is_none() {
                                start = Some(Instant::now());
                            }
                            if let Some(span) = text.lines[0].spans.get_mut(position) {
                                text.lines[0].spans[position] =
                                    span.clone().bg(Color::Reset).fg(CORRECT);
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
                let end = start.unwrap().elapsed().as_nanos();

                let race = Race::new(hits, misses, end, splits.clone());
                total_words += race.words(race.length);
                total_time += race.minutes();

                last_race = Some(race);

                break 'race;
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_race(c: char) -> io::Result<Keystroke> {
    if event::poll(std::time::Duration::from_millis(4))? {
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

fn draw_keyboard(frame: &mut Frame, area: &Rect, next_c: &str, layout: Option<&String>) {
    let shift = false;
    let layout = get_keyboard_layout(layout, shift);

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
    for (i, c) in layout.rows[0].iter().enumerate() {
        let mut paragraph = Paragraph::new(c.clone())
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);

        if *c == next_c {
            paragraph = paragraph.style(Style::default().fg(CORRECT));
        }

        (*frame).render_widget(paragraph, row_1[i]);
    }

    let row_2 = Layout::new(Direction::Horizontal, constraints).split(keyboard[1]);

    for (i, c) in layout.rows[1].iter().enumerate() {
        let mut paragraph = Paragraph::new(c.clone())
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);

        if *c == next_c {
            paragraph = paragraph.style(Style::default().fg(CORRECT));
        }

        (*frame).render_widget(paragraph, row_2[i]);
    }

    let row_3 = Layout::new(Direction::Horizontal, constraints).split(keyboard[2]);

    for (i, c) in layout.rows[2].iter().enumerate() {
        let mut paragraph = Paragraph::new(c.clone())
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
        if event::poll(std::time::Duration::from_millis(4))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(key.code);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn is_shifted() -> io::Result<bool> {
    loop {
        if event::poll(std::time::Duration::from_millis(4))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press
                    && key.modifiers.contains(KeyModifiers::SHIFT)
                {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
    }
}
