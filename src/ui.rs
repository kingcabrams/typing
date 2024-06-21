use crate::types::{Race, ASCII_ART_1, ASCII_ART_2, TITLE};
use ratatui::{prelude::*, widgets::*};

use super::draw_keyboard;

pub fn title(
    frame: &mut Frame,
    last_race: Option<Race>,
    total_words: f64,
    total_time: f64,
    username: &String,
    show_results: bool,
) {
    let areas = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(2),
            Constraint::Percentage(5),
            Constraint::Percentage(28),
            Constraint::Percentage(40),
            Constraint::Percentage(4),
            Constraint::Percentage(4),
            Constraint::Percentage(2),
            Constraint::Percentage(4),
        ],
    )
    .split(frame.size());

    if last_race.is_none() {
        frame.render_widget(
            Paragraph::new("(s) start | (q) quit").alignment(Alignment::Center),
            areas[1],
        );
    } else {
        frame.render_widget(
            Paragraph::new("(s) start | (q) quit | (r) results").alignment(Alignment::Center),
            areas[1],
        );
    }

    frame.render_widget(
        Paragraph::new(ASCII_ART_1)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White)),
        areas[2],
    );

    frame.render_widget(
        Paragraph::new(ASCII_ART_2)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White)),
        areas[3],
    );

    frame.render_widget(
        Paragraph::new(format!("{username}")).alignment(Alignment::Center),
        areas[5],
    );

    if total_words > 0.0 {
        frame.render_widget(
            Paragraph::new(format!(
                "Session average: {:.0} wpm",
                total_words / total_time
            ))
            .alignment(Alignment::Center),
            areas[7],
        );
    }

    if let Some(race) = last_race {
        if show_results {
            results(frame, race);
        }
    }
}

pub fn race(
    frame: &mut Frame,
    quote_name: String,
    paragraph: Paragraph,
    layout: Option<&String>,
    next_char: String,
) {
    let areas = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(5),
            Constraint::Length(12),
            Constraint::Fill(1),
        ],
    )
    .split(frame.size());

    let text = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ],
    )
    .split(areas[1]);

    let textboxes = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(20), Constraint::Percentage(80)],
    )
    .split(text[1]);

    frame.render_widget(
        Paragraph::new(format!("## {quote_name}"))
            .style(Style::default().add_modifier(Modifier::BOLD).fg(TITLE)),
        textboxes[0],
    );

    frame.render_widget(paragraph.wrap(Wrap { trim: false }).alignment( Alignment::Center ), textboxes[1]);
    let keyboard_width = 60;

    let split = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Min((areas[2].width - keyboard_width) / 2),
            Constraint::Min(60),
            Constraint::Min((areas[2].width - keyboard_width) / 2),
        ],
    )
    .split(areas[3]);

    draw_keyboard(frame, &split[1], &next_char, layout);
}

pub fn results(frame: &mut Frame, race: Race) {
    let area = centered_rect(90, frame.size());
    let popup_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(2),
        ],
    )
    .split(area);

    let stats = Layout::new(
        Direction::Vertical,
        [
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ],
    )
    .split(popup_layout[1]);

    let block = Block::bordered();

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);

    let wpm = race.wpm_data();
    let raw = race.raw_data();

    frame.render_widget(
        Paragraph::new(format!("wpm",)).alignment(Alignment::Center),
        stats[1],
    );

    frame.render_widget(
        Paragraph::new(format!("{:.0}", f64::round(race.wpm()),)).alignment(Alignment::Center),
        stats[2],
    );

    frame.render_widget(
        Paragraph::new(format!("acc",)).alignment(Alignment::Center),
        stats[4],
    );

    frame.render_widget(
        Paragraph::new(format!("{:.2}%", race.accuracy())).alignment(Alignment::Center),
        stats[5],
    );

    let (mut min_first, mut max_first) = (f64::MAX, f64::MIN);
    let (mut min_second, mut max_second) = (f64::MAX, f64::MIN);

    for (first, second) in wpm.iter() {
        min_first = f64::min(min_first, *first);
        min_second = f64::min(min_second, *second);
        max_first = f64::max(max_first, *first);
        max_second = f64::max(max_second, *second);
    }

    let graph = Chart::new(vec![
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().white())
            .data(&raw),
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().green())
            .data(&wpm),
    ])
    .block(Block::new())
    .x_axis(
        Axis::default()
            .style(Style::default().white())
            .bounds([min_first, max_first])
            .labels(vec![
                format!("{:.0}", min_first).into(),
                format!("{:.0}", max_first).into(),
            ]),
    )
    .y_axis(
        Axis::default()
            .style(Style::default().white())
            .bounds([min_second, max_second])
            .labels(vec![
                format!("{:.0}", min_second).into(),
                format!("{:.0}", max_second).into(),
            ]),
    );

    let graph_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ],
    )
    .split(popup_layout[3]);

    frame.render_widget(graph, graph_layout[1]);
}

fn centered_rect(percent_x: u16, r: Rect) -> Rect {
    let percent_y = 11;
    let popup_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(percent_y),
        Constraint::Fill(1),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
