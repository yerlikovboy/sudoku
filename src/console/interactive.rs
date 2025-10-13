use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::game::cell::Cell;
use crate::game::puzzle::Puzzle;

struct CursorPosition {
    row: usize,
    col: usize,
}

impl CursorPosition {
    fn new() -> Self {
        CursorPosition { row: 0, col: 0 }
    }

    fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.row < 8 {
            self.row += 1;
        }
    }

    fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.col < 8 {
            self.col += 1;
        }
    }
}

fn render_puzzle(
    puzzle: &Puzzle,
    cursor_pos: &CursorPosition,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    // Clear screen and move to top
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let grid = puzzle.grid();

    // Build the complete output line by line
    let mut lines: Vec<String> = Vec::new();

    // Title
    lines.push("Sudoku - Interactive Mode".to_string());
    lines.push("Arrow keys: move | 1-9: fill | 0/Del: clear | q: quit".to_string());
    lines.push("".to_string());

    // Render the grid with simple ASCII
    for row in 0..9 {
        // Top border for 3x3 blocks
        if row % 3 == 0 {
            lines.push("  +----------+----------+----------+".to_string());
        }

        let mut line = String::from("  | ");

        for col in 0..9 {
            let cell = &grid[row * 9 + col];
            let value = cell.value();
            let is_selected = row == cursor_pos.row && col == cursor_pos.col;

            // Determine the character to display
            let ch = if value.is_none() {
                '.'
            } else {
                let n = value.unwrap();
                char::from_digit(n as u32, 10).unwrap()
            };

            // Format cell with or without selection brackets
            if is_selected {
                line.push('[');
                line.push(ch);
                line.push(']');
            } else {
                line.push(ch);
            }

            // Spacing and column separators
            if col % 3 == 2 {
                if col < 8 {
                    line.push_str(" | ");
                } else {
                    line.push_str(" ");
                }
            } else {
                line.push(' ');
            }
        }

        line.push('|');
        lines.push(line);
    }

    // Bottom border
    lines.push("  +----------+----------+----------+".to_string());
    lines.push("".to_string());

    // Status line
    lines.push(format!("  Position: Row {}, Col {}", cursor_pos.row + 1, cursor_pos.col + 1));

    // Now print everything with proper line endings and colors
    for (line_idx, line) in lines.iter().enumerate() {
        // Move cursor to start of line
        write!(stdout, "\r")?;

        // For grid lines (rows 3-13), apply colors to individual characters
        if line_idx >= 3 && line_idx < 3 + 9 + 4 && line.contains('|') && !line.contains('+') {
            // This is a grid row with cells
            let row = if line_idx < 3 + 1 + 3 {
                line_idx - 4
            } else if line_idx < 3 + 2 + 6 {
                line_idx - 5
            } else {
                line_idx - 6
            };

            // Print with colors
            write!(stdout, "  | ")?;
            for col in 0..9 {
                let cell = &grid[row * 9 + col];
                let value = cell.value();
                let is_selected = row == cursor_pos.row && col == cursor_pos.col;

                let (color, ch) = if value.is_none() {
                    (Color::DarkGrey, '.')
                } else {
                    let n = value.unwrap();
                    let color = if cell.has_conflicts() {
                        Color::Red
                    } else if cell.is_clue() {
                        Color::Cyan
                    } else {
                        Color::Yellow
                    };
                    (color, char::from_digit(n as u32, 10).unwrap())
                };

                if is_selected {
                    execute!(stdout, SetForegroundColor(Color::Green))?;
                    write!(stdout, "[")?;
                    execute!(stdout, SetForegroundColor(color))?;
                    write!(stdout, "{}", ch)?;
                    execute!(stdout, SetForegroundColor(Color::Green))?;
                    write!(stdout, "]")?;
                    execute!(stdout, ResetColor)?;
                } else {
                    execute!(stdout, SetForegroundColor(color))?;
                    write!(stdout, " {} ", ch)?;
                    execute!(stdout, ResetColor)?;
                }

                if col % 3 == 2 {
                    if col < 8 {
                        write!(stdout, "| ")?;
                    }
                }
            }
            write!(stdout, "|\r\n")?;
        } else {
            // Regular line, just print it
            write!(stdout, "{}\r\n", line)?;
        }
    }

    stdout.flush()?;
    Ok(())
}

pub fn play(puzzle: &mut Puzzle) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut cursor_pos = CursorPosition::new();

    // Enter alternate screen and enable raw mode
    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let result = game_loop(puzzle, &mut cursor_pos);

    // Clean up: disable raw mode and leave alternate screen
    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;

    result
}

fn game_loop(
    puzzle: &mut Puzzle,
    cursor_pos: &mut CursorPosition,
) -> io::Result<()> {
    loop {
        render_puzzle(puzzle, cursor_pos)?;

        // Check if puzzle is completed
        if puzzle.is_completed() {
            println!("\n  Congratulations! You've solved the puzzle!");
            println!("  Press any key to exit...");
            event::read()?;
            break;
        }

        // Handle input
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            ..
        }) = event::read()?
        {
            match code {
                // Arrow keys for movement
                KeyCode::Up => cursor_pos.move_up(),
                KeyCode::Down => cursor_pos.move_down(),
                KeyCode::Left => cursor_pos.move_left(),
                KeyCode::Right => cursor_pos.move_right(),

                // Number input (1-9)
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap() as u8;
                    let cell = Cell::user_request(cursor_pos.row, cursor_pos.col,
                        if digit == 0 { None } else { Some(digit) });

                    let _ = puzzle.update_cell(&cell);
                }

                // Delete/Backspace to clear cell
                KeyCode::Delete | KeyCode::Backspace => {
                    let cell = Cell::user_request(cursor_pos.row, cursor_pos.col, None);
                    let _ = puzzle.update_cell(&cell);
                }

                // 'u' for undo (TODO: implement undo functionality)
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    // TODO: implement undo
                }

                // 'q' or Ctrl+C to quit
                KeyCode::Char('q') | KeyCode::Char('Q') => break,
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break,

                _ => {}
            }
        }
    }

    Ok(())
}
