use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{self, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    DefaultTerminal, Frame,
};

struct TabsState<'a> {
    titles: Vec<&'a str>,
    index: usize,
}

impl<'a> TabsState<'a> {
    fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }

    fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

struct App<'a> {
    title: &'a str,
    tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    fn new(title: &'a str) -> Self {
        Self {
            title,
            tabs: TabsState::new(vec!["tab1", "tab2", "tab3"]),
        }
    }

    fn on_right(&mut self) {
        self.tabs.next();
    }

    fn on_left(&mut self) {
        self.tabs.previous();
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let first_arg = std::env::args().nth(1).unwrap_or_default();
    let terminal = ratatui::init();
    let app_result = run(terminal, &first_arg);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal, first_arg: &str) -> Result<()> {
    let mut should_quit = false;
    let mut app = App::new("Termion demo");
    while !should_quit {
        terminal.draw(|f| {
            match first_arg {
                "layout" => layout(f),
                _ => draw(f, &mut app),
            };
        })?;
        should_quit = handle_events(&mut app)?;
    }
    Ok(())
}

fn handle_events(app: &mut App) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Left => {
                    app.on_left();
                }
                KeyCode::Right => {
                    app.on_right();
                }
                _ => {}
            }
        }
    }
    Ok(false)
}

fn layout(frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);

    let horizontal = Layout::horizontal([Constraint::Ratio(1, 2); 2]);
    let [title_bar, main_area, status_bar] = vertical.areas(frame.area());
    let [left, right] = horizontal.areas(main_area);
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        title_bar,
    );

    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        status_bar,
    );

    frame.render_widget(Block::bordered().title("Left"), left);
    frame.render_widget(Block::bordered().title("right"), right);
}

fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            text::Line::from(Span::styled(
                *t,
                Style::default().fg(ratatui::style::Color::Green),
            ))
        })
        .collect::<Tabs>()
        .block(Block::bordered().title(app.title))
        .highlight_style(Style::default().fg(ratatui::style::Color::Yellow))
        .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(frame, app, chunks[1]),
        1 => draw_second_tab(frame, app, chunks[1]),
        2 => draw_third_tab(frame, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab(frame: &mut Frame, _: &mut App, area: Rect) {
    let p = Paragraph::new("tab1".to_string()).block(Block::bordered().title("测试"));
    frame.render_widget(p, area)
}

fn draw_second_tab(frame: &mut Frame, _: &mut App, area: Rect) {
    let p = Paragraph::new("tab2".to_string()).block(Block::bordered().title("测试"));
    frame.render_widget(p, area)
}

fn draw_third_tab(frame: &mut Frame, _: &mut App, area: Rect) {
    let p = Paragraph::new("tab3".to_string()).block(Block::bordered().title("测试"));
    frame.render_widget(p, area)
}
