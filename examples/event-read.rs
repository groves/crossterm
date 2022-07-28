//! Demonstrates how to block read events.
//!
//! cargo run --example event-read

use std::io::stdout;

use crossterm::event::poll;
use crossterm::{
    cursor::position,
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture,
        Event, KeyCode,
    },
    execute,
    input::{ read, Input, EnableBracketedPaste, DisableBracketedPaste },
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::time::Duration;

const HELP: &str = r#"Blocking read()
 - Keyboard, mouse, focus and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

fn print_events() -> Result<()> {
    loop {
        // Blocking read
        let input = read()?;

        match input {
            Input::Event(event) => {
                println!("Event: {:?}\r", event);
                if event == Event::Key(KeyCode::Char('c').into()) {
                    println!("Cursor position: {:?}\r", position());
                }

                if let Event::Resize(_, _) = event {
                    let (original_size, new_size) = flush_resize_events(event);
                    println!("Resize from: {:?}, to: {:?}", original_size, new_size);
                }

                if event == Event::Key(KeyCode::Esc.into()) {
                    break;
                }
            },
            Input::Paste(data) => {
                println!("Pasted {:?}", data);
            }
        }

    }

    Ok(())
}

// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
fn flush_resize_events(event: Event) -> ((u16, u16), (u16, u16)) {
    if let Event::Resize(x, y) = event {
        let mut last_resize = (x, y);
        while let Ok(true) = poll(Duration::from_millis(50)) {
            if let Ok(Input::Event(Event::Resize(x, y))) = read() {
                last_resize = (x, y);
            }
        }

        return ((x, y), last_resize);
    }
    ((0, 0), (0, 0))
}

fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableBracketedPaste, EnableFocusChange, EnableMouseCapture)?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture)?;

    disable_raw_mode()
}
