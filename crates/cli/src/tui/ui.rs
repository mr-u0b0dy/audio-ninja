// SPDX-License-Identifier: Apache-2.0

//! UI rendering logic

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
};

use super::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_header(f, chunks[0], app);
    draw_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let screens = vec!["Dashboard", "Speakers", "Layout", "Transport", "Calibration"];
    let current = match app.current_screen {
        Screen::Dashboard => 0,
        Screen::Speakers => 1,
        Screen::Layout => 2,
        Screen::Transport => 3,
        Screen::Calibration => 4,
    };

    let tabs = Tabs::new(screens)
        .block(Block::default().borders(Borders::BOTTOM).title("Audio Ninja"))
        .select(current)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        );

    f.render_widget(tabs, area);
}

fn draw_content(f: &mut Frame, area: Rect, app: &App) {
    match app.current_screen {
        Screen::Dashboard => draw_dashboard(f, area, app),
        Screen::Speakers => draw_speakers(f, area, app),
        Screen::Layout => draw_layout(f, area, app),
        Screen::Transport => draw_transport(f, area, app),
        Screen::Calibration => draw_calibration(f, area, app),
    }
}

fn draw_dashboard(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title("Dashboard")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(inner);

    let mut status_text = vec![Line::from(Span::styled(
        "Daemon Status",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))];

    if let Some(status) = &app.status {
        status_text.push(Line::from(format!("{}", status)));
    } else {
        status_text.push(Line::from(Span::styled(
            "Loading...",
            Style::default().fg(Color::Yellow),
        )));
    }

    let status_para = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::LEFT))
        .wrap(Wrap { trim: true });
    f.render_widget(status_para, chunks[0]);

    let mut stats_text = vec![Line::from(Span::styled(
        "Statistics",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))];

    if let Some(stats) = &app.stats {
        stats_text.push(Line::from(format!("{}", stats)));
    } else {
        stats_text.push(Line::from(Span::styled(
            "Loading...",
            Style::default().fg(Color::Yellow),
        )));
    }

    let stats_para = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::LEFT))
        .wrap(Wrap { trim: true });
    f.render_widget(stats_para, chunks[1]);
}

fn draw_speakers(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title("Speakers")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut text = vec![Line::from(Span::styled(
        "Connected Speakers",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))];

    if let Some(speakers) = &app.speakers {
        text.push(Line::from(format!("{}", speakers)));
    } else {
        text.push(Line::from(Span::styled(
            "No speakers connected. Press 'd' to discover.",
            Style::default().fg(Color::Yellow),
        )));
    }

    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::LEFT))
        .wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

fn draw_layout(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title("Layout")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut text = vec![Line::from(Span::styled(
        "Current Layout",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))];

    if let Some(layout) = &app.layout {
        text.push(Line::from(format!("{}", layout)));
    } else {
        text.push(Line::from(Span::styled(
            "Loading...",
            Style::default().fg(Color::Yellow),
        )));
    }

    text.push(Line::from(""));
    text.push(Line::from("Available presets: stereo, 5.1, 7.1"));

    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::LEFT))
        .wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

fn draw_transport(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title("Transport")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut text = vec![Line::from(Span::styled(
        "Playback Control",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))];

    if let Some(status) = &app.transport_status {
        text.push(Line::from(format!("{}", status)));
    } else {
        text.push(Line::from(Span::styled(
            "Loading...",
            Style::default().fg(Color::Yellow),
        )));
    }

    text.push(Line::from(""));
    text.push(Line::from("Controls: [P]lay  [S]top  [R]esume"));

    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::LEFT))
        .wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

fn draw_calibration(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title("Calibration")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut text = vec![Line::from(Span::styled(
        "Calibration Status",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))];

    if let Some(status) = &app.calibration_status {
        text.push(Line::from(format!("{}", status)));
    } else {
        text.push(Line::from(Span::styled(
            "No active calibration",
            Style::default().fg(Color::Yellow),
        )));
    }

    text.push(Line::from(""));
    text.push(Line::from("Controls: [C]alibrate  [A]pply"));

    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::LEFT))
        .wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let help_text = if let Some(error) = &app.error_message {
        Line::from(Span::styled(
            format!("Error: {}", error),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ))
    } else {
        Line::from(vec![
            Span::raw("Navigation: "),
            Span::styled("[←/→]", Style::default().fg(Color::Yellow)),
            Span::raw(" Tabs • "),
            Span::styled("[↑/↓]", Style::default().fg(Color::Yellow)),
            Span::raw(" Select • "),
            Span::styled("[q]", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit • "),
            Span::styled("[r]", Style::default().fg(Color::Yellow)),
            Span::raw(" Refresh"),
        ])
    };

    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::TOP))
        .alignment(Alignment::Left);

    f.render_widget(footer, area);
}
