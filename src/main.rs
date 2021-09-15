use std::io;

use crossterm::{
    cursor,
    event::{read, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

use editor::{editor::Editor, operation::Operation};

fn main() -> crossterm::Result<()> {
    let mut editor = Editor::new("./test");

    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        Print(&editor.text),
        cursor::MoveTo(0, 0),
        EnableMouseCapture,
    )?;

    enable_raw_mode()?;

    // Should there be a cursor, line_num and pos
    // in the editor, and the frontend should only mirror it?
    let mut x = 0;
    let mut y = 0;

    loop {
        let current_char = if x > 0 {
            Some(editor.text.line(y as usize).char((x - 1) as usize))
        } else {
            None
        };

        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Esc => break,
                KeyCode::Backspace => {
                    if let Some(c) = current_char {
                        editor.run_operation(Operation::Remove((x - 1) as usize, c.into()));
                    }
                    if x > 0 {
                        x -= 1
                    }
                }
                KeyCode::Char(c) => {
                    editor.run_operation(Operation::Insert(x as usize, c.into()));
                    x += 1;
                }

                KeyCode::Up => {
                    if y > 0 {
                        y -= 1
                    }
                }
                KeyCode::Down => y += 1,
                KeyCode::Left => {
                    if x > 0 {
                        x -= 1
                    }
                }
                KeyCode::Right => x += 1,

                _ => {}
            },
            Event::Mouse(event) => match event.kind {
                MouseEventKind::Drag(_) | MouseEventKind::Down(_) => {
                    x = event.column;
                    y = event.row;
                }
                _ => {}
            },
            Event::Resize(_, _) => {}
        }

        // This renders incorrectly
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            Print(&editor.text),
            cursor::MoveTo(x, y),
        )?;

        // This is necessary in order to properly render text,
        // but it doesnt work very well
        // stdout.execute(Clear(ClearType::All))?;
        // for (idx, line) in editor.text.lines().enumerate() {
        //     execute!(stdout, cursor::MoveTo(0, idx as u16), Print(line))?;
        // }
        // stdout.execute(cursor::MoveTo(x, y))?;
    }

    disable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0),)?;

    Ok(())
}
