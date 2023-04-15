use color_eyre::Result;
use crossterm::{
    event::{self, EnableBracketedPaste, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use editor::{Backend as EditorBackend, Frontend as EditorFrontend, Direction};
use std::{io::{self}, time::Duration};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

mod editor;

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
    let editor = &mut EditorBackend::new();

    loop {
        terminal.draw(|f| {
            let frontend = EditorFrontend::new(editor);
            let area = f.size();
            f.render_widget(frontend, area);
            let (y, x) = editor.position();
            let msg = "Could not convert editor position to terminal position";
            let x: u16 = x.try_into().expect(msg);
            let y: u16 = y.try_into().expect(msg);
            f.set_cursor(x, y);
        })?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(KeyEvent{code, modifiers: _, kind: _, state: _}) => match code {
                    KeyCode::Char(ch) => editor.insert(ch),
                    KeyCode::Backspace => editor.remove_char(),
                    KeyCode::Up => editor.move_cursor(Direction::Up),
                    KeyCode::Right => editor.move_cursor(Direction::Right),
                    KeyCode::Down => editor.move_cursor(Direction::Down),
                    KeyCode::Left => editor.move_cursor(Direction::Left),
                    KeyCode::Enter => editor.insert('\n'),
                    KeyCode::Esc => {
                        std::fs::write("out.txt", editor.to_string())?;
                        return Ok(());
                    },
                    _ => {},
                },
                Event::Paste(text) => editor.insert_str(&text.replace('\r', "\n")),
                _ => (),
            }
        }
    }
}

#[derive(Debug, Default)]
struct Card {
    title: String,
    tags: Vec<String>,
    body: String,
}
