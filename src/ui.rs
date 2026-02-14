use crate::{
    app::App,
    helpers::color_helper::{EXT_DEFAULT_CLR, get_ext_color},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

const LIST_POS: usize = 0;
const PATH_PREVIEW: usize = 1;
const SEARCH_BAR_POS: usize = 2;
const STATUS_BAR_POS: usize = 3;

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // List
            Constraint::Length(3), // Path preview
            Constraint::Length(3), // Search bar
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    let block = Block::bordered()
        .title(format!(
            " Found {} apps in {:.2?} ",
            app.items_found, app.scan_duration
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    render_list(app, frame, block, chunks[LIST_POS]);
    render_search_bar(app, frame, chunks[SEARCH_BAR_POS]);
    render_path_preview(app, frame, chunks[PATH_PREVIEW]);
    render_status_bar(frame, chunks[STATUS_BAR_POS]);
}

fn render_search_bar(app: &mut App, frame: &mut Frame, area: Rect) {
    let search_bar = Paragraph::new(app.filter_query.as_str())
        .block(
            Block::bordered()
                .title(" Type to filter ")
                .border_style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Left);

    frame.render_widget(search_bar, area);
}

fn render_list(app: &mut App, frame: &mut Frame, block: Block, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_apps
        .iter()
        .map(|app_entry| {
            let ext = app_entry
                .path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            let color = get_ext_color(ext);

            let content = Line::from(vec![Span::styled(
                app_entry.name.as_str(),
                Style::default().fg(color),
            )]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).bold())
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_path_preview(app: &mut App, frame: &mut Frame, area: Rect) {
    let preview_width = area.width.saturating_sub(2) as usize;

    let selected_entry = app
        .list_state
        .selected()
        .and_then(|i| app.filtered_apps.get(i));

    let color = if let Some(entry) = selected_entry {
        let ext = entry
            .path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        get_ext_color(ext)
    } else {
        EXT_DEFAULT_CLR
    };

    let selected_path = selected_entry
        .map(|entry| entry.path.to_string_lossy().to_string())
        .unwrap_or_else(|| "No selection".to_string());

    let path_len = selected_path.len();
    let display_path = if path_len > preview_width && preview_width > 3 {
        let truncate_pos = path_len - (preview_width - 3);
        format!("...{}", &selected_path[truncate_pos..])
    } else {
        selected_path
    };

    let title = match app.clipboard_status_msg.as_str() {
        "" => " Selected Path ".to_string(),
        msg => {
            format!(" Selected Path - {msg}")
        }
    };

    let path_preview = Paragraph::new(display_path)
        .block(Block::bordered().title(title))
        .style(Style::default().fg(color))
        .alignment(Alignment::Left);

    frame.render_widget(path_preview, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect) {
    let options = vec![
        ("Enter", "Open", Color::LightGreen),
        ("CTRL+C", "Copy Path", Color::White),
        ("CTRL+O", "Explorer", Color::LightYellow),
        ("ESC", "Quit", Color::LightRed),
    ];

    let spans: Vec<Span> = options
        .into_iter()
        .flat_map(|(key, desc, color)| {
            vec![
                Span::styled(
                    format!("{}: ", key),
                    Style::default().bg(Color::default()).fg(color),
                ),
                Span::styled(
                    format!("{}", desc),
                    Style::default().bg(Color::default()).fg(color),
                ),
                Span::styled(
                    " | ",
                    Style::default().bg(Color::default()).fg(Color::DarkGray),
                ),
            ]
        })
        .collect();

    let status_line = Line::from(spans);

    let paragraph = Paragraph::new(status_line)
        .style(Style::default().bg(Color::default()))
        .left_aligned();

    frame.render_widget(paragraph, area);
}
