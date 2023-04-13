use color_eyre::Result;
use crossterm::{
    event::{self, EnableBracketedPaste, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self};
use tui::{
    backend::CrosstermBackend,
    style::Style,
    text::{Span, Spans},
    widgets::Paragraph,
    Terminal,
};

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut text = ropey::Rope::new();
    let mut index = 0_usize;

    loop {
        let spans = Spans::from(
            text.lines()
                .filter_map(|line| line.as_str().map(|str| Span::styled(str, Style::default())))
                .collect::<Vec<Span>>(),
        );

        terminal.draw(|f| {
            let size = f.size();
            let paragraph = Paragraph::new(spans).wrap(tui::widgets::Wrap { trim: false });
            f.render_widget(paragraph, size);
        })?;

        match event::read()? {
            Event::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                if let KeyCode::Char(c) = key.code {
                    text.insert_char(index, c);
                    index += 1;
                }

                if key.code == KeyCode::Enter {
                    text.insert_char(index, '\n');
                    index += 1;
                }

                if key.code == KeyCode::Backspace {
                    text.remove(index - 1..index);
                    index -= 1;
                }
            }
            Event::Paste(string) => {
                text.insert(index, &string);
                index += string.len()
            }
            _ => (),
        }
    }
}

fn reformat_string(input: &str, max_length: usize) -> String {
    let mut current_chunk = String::new();
    let mut chunks: Vec<String> = Vec::new();
    let max_length = max_length - 2;

    for c in input.chars() {
        current_chunk.push(c);

        if current_chunk.len() == max_length || c == '\n' {
            if c != '\n' {
                current_chunk.push(' ');
                current_chunk.push('↩');
                current_chunk.push('\n');
            }
            chunks.push(current_chunk.clone());
            current_chunk.clear();
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks.concat()
}

#[derive(Debug, Default)]
struct Card {
    title: String,
    tags: Vec<String>,
    body: String,
}
