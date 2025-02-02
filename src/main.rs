/*

TUI example in Rust 

--

see : https://ratatui.rs/examples/widgets/table/

cargo run --bin ru_ratatui1

*/

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table}, DefaultTerminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal : DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
        vec![
            "Alice".to_string(),
            "30".to_string(),
            "New York".to_string(),
        ],
        vec!["Bob".to_string(), "25".to_string(), "London".to_string()],
        vec!["Charlie".to_string(), "35".to_string(), "Paris".to_string()],
        vec![
            "Joana".to_string(),
            "20".to_string(),
            "Budapest".to_string(),
        ],
    ];

    let mut selected_row = 1; // Start at the first data row (excluding header)
    let mut selected_col = 0;

    loop {
        terminal.draw(|f| {
            let widths = data.iter().fold(vec![0; data[0].len()], |mut acc, row| {
                for (i, cell) in row.iter().enumerate() {
                    acc[i] = acc[i].max(cell.len());
                }
                acc
            });

            let rows = data.iter().enumerate().map(|(row_index, row)| {
                let cells = row.iter().enumerate().map(|(col_index, cell)| {
                    let style = if row_index == selected_row && col_index == selected_col {
                        Style::default().bg(Color::LightYellow).fg(Color::Black)
                    // Highlight selected cell
                    } else {
                        Style::default()
                    };
                    Cell::from(cell.clone()).style(style)
                });
                Row::new(cells).height(1)
            });

            //let twidths = [Constraint::Length(15), Constraint::Length(5),Constraint::Length(10)];

            let twidths: Vec<Constraint> = widths
                .iter()
                .map(|c| Constraint::Length((*c as u16) + 10))
                .collect();
            let table = Table::new(rows, twidths)
                .block(Block::default().title("User Data").borders(Borders::ALL))
                .column_spacing(1);

            f.render_widget(table, f.area());
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected_row = selected_row.saturating_sub(1).max(1)
                    } // Don't go above the header row
                    KeyCode::Down | KeyCode::Char('j') => {
                        selected_row = (selected_row + 1).min(data.len() - 1)
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        selected_col = selected_col.saturating_sub(1)
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        selected_col = (selected_col + 1).min(data[0].len() - 1)
                    }
                    _ => {}
                }
            }
        }
    }


    Ok(())
}
