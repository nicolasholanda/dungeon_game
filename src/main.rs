use std::{
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use dungeon_game::Solution;

struct App {
    dungeon: Vec<Vec<i32>>,
    path: Vec<(usize, usize)>,
    current_step: usize,
    current_hp: i32,
    min_hp: i32,
    running: bool,
}

impl App {
    fn new() -> Self {
        let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        let (min_hp, path) = Solution::calculate_minimum_hp(dungeon.clone());
        Self {
            dungeon,
            path,
            current_step: 0,
            current_hp: min_hp,
            min_hp,
            running: true,
        }
    }

    fn tick(&mut self) {
        if self.current_step < self.path.len() - 1 {
            self.current_step += 1;
            let (r, c) = self.path[self.current_step];
            self.current_hp -= self.dungeon[r][c];
        }
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();

    while app.running {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.running = false;
                }
            }
        }
        app.tick();
    }

    // Restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(frame.size());

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    frame.render_widget(
        Paragraph::new(format!("Initial HP needed: {}", app.min_hp))
            .block(Block::default().title("Game Info").borders(Borders::ALL)),
        header_layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!("Current HP: {}", app.current_hp))
            .block(Block::default().title("Player Stats").borders(Borders::ALL)),
        header_layout[1],
    );

    let rows = app.dungeon.len();
    let cols = app.dungeon[0].len();

    let mut table_rows = Vec::new();
    for r in 0..rows {
        let mut cells = Vec::new();
        for c in 0..cols {
            let (pr, pc) = app.path[app.current_step];
            let cell_text = format!("{}", app.dungeon[r][c]);
            let mut cell = Cell::from(cell_text);
            if r == pr && c == pc {
                cell = cell.style(Style::default().bg(Color::Yellow).fg(Color::Black));
            }
            cells.push(cell);
        }
        table_rows.push(Row::new(cells).height(3));
    }

    let widths = (0..cols).map(|_| Constraint::Percentage(100 / cols as u16)).collect::<Vec<_>>();
    let table = Table::new(table_rows, widths)
        .block(Block::default().title("Dungeon").borders(Borders::ALL))
        .column_spacing(1);

    frame.render_widget(table, main_layout[0]);
}
