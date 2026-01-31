use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

use crate::app::App;

const LIST_POS: usize = 0;
const SEARCH_BAR_POS: usize = 1;
const STATUS_SECTION_POS: usize = 2;

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // List
            Constraint::Length(3), //Search bar
            Constraint::Length(3), // Status
        ])
        .split(frame.area());

    let block = Block::bordered()
        .title("app-launcher")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    render_list(app, frame, block, chunks[LIST_POS]);
    render_search_bar(app, frame, chunks[SEARCH_BAR_POS]);
    render_status_section(app, frame, chunks[STATUS_SECTION_POS]);
}

fn render_search_bar(app: &mut App, frame: &mut Frame, area: Rect) {
    let search_bar = Paragraph::new(app.filter_query.as_str())
        .block(
            Block::bordered()
                .title(" Search (Type to filter) ")
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Left);

    frame.render_widget(search_bar, area);
}

fn render_list(app: &mut App, frame: &mut Frame, block: Block, area: Rect) {
    let items: Vec<ListItem> = app
        .launchable_apps
        .iter()
        .filter(|name| {
            app.filter_query.is_empty()
                || name
                    .to_lowercase()
                    .contains(&app.filter_query.to_lowercase())
        })
        .map(|app_name| ListItem::new(app_name.as_str()))
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_status_section(app: &mut App, frame: &mut Frame, area: Rect) {
    let status_text = format!(
        "Found {} apps in {:.2?}",
        app.items_found, app.scan_duration
    );

    let status = Paragraph::new(status_text)
        .block(Block::bordered().title("Status"))
        .alignment(Alignment::Center);

    frame.render_widget(status, area);
}
