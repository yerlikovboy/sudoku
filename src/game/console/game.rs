use std::char;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::Read;

use crate::game::cell::Cell;
use crate::game::puzzle::Puzzle;

use crate::game::console::utils;

const LINE_FEED: u8 = 10;
const SPACE: u8 = 32;
const END_OF_TRANSMISSION: u8 = 4;

enum ByteType {
    DIGIT,
    SPC,
    NEWLINE,
}

impl Display for ByteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t_name = match self {
            ByteType::DIGIT => "digit",
            ByteType::SPC => "space",
            ByteType::NEWLINE => "none",
        };
        f.write_str(t_name)
    }
}

enum UserRequest {
    Move(Cell),
    UndoMove,
    Help,
    Quit,
    Undefined,
}

impl Display for UserRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let retval = match self {
            UserRequest::Move(c) => f.write_fmt(format_args!("Move: [{}]", c)),
            UserRequest::UndoMove => f.write_str("undo last move"),
            UserRequest::Help => f.write_str("display help"),
            UserRequest::Quit => f.write_str("quit"),
            UserRequest::Undefined => f.write_str("undefined"),
        };
        retval
    }
}

fn get_request() -> UserRequest {
    let mut prev = ByteType::NEWLINE;

    println!("Enter h for help, Ctrl-D or q+Enter to stop");
    let mut stdin_bytes = io::stdin().bytes().peekable();
    let mut vals: Vec<u8> = Vec::new();

    loop {
        // read in one byte
        let b_val: u8 = stdin_bytes
            .next()
            .unwrap_or(Ok(END_OF_TRANSMISSION)) // interpret None as Ctrl+D
            .unwrap();

        match b_val {
            END_OF_TRANSMISSION => return UserRequest::Quit,
            LINE_FEED => {
                // reset everything
                vals.clear();
                prev = ByteType::NEWLINE;
            }
            SPACE => {
                prev = ByteType::SPC;
                continue;
            }
            _ => {
                // do not check if there is anything more in stdin ...
                if b_val.is_ascii_alphabetic() {
                    if vals.len() != 0 {
                        println!("error: in the middle of reading users move");
                        while *stdin_bytes.peek().unwrap().as_ref().unwrap() != LINE_FEED {
                            stdin_bytes.next();
                        }
                        continue;
                    }
                    let user_opt = char::from_u32(b_val.into()).unwrap_or('_');
                    return match user_opt {
                        'q' => UserRequest::Quit,
                        'h' => UserRequest::Help,
                        'u' => UserRequest::UndoMove,
                        _ => UserRequest::Undefined,
                    };
                }

                if b_val.is_ascii_digit() {
                    // check previous character ...
                    match prev {
                        ByteType::DIGIT => {
                            println!("error -> previous byte also digit");
                            while *stdin_bytes.peek().unwrap().as_ref().unwrap() != LINE_FEED {
                                stdin_bytes.next();
                            }
                        }
                        ByteType::SPC | ByteType::NEWLINE => {
                            let v: u8 = b_val - 48;
                            vals.push(v);
                        }
                    };
                    if vals.len() == 3 {
                        return UserRequest::Move(
                            utils::user_move(vals[0], vals[1], vals[2]).unwrap(),
                        );
                    }
                    prev = ByteType::DIGIT;
                }
            }
        }
    }
}

pub fn play(puzzle: &mut Puzzle) {
    loop {
        utils::print_console(puzzle);
        println!("Please enter your next move (row column value) or Ctrl-C to quit: ");

        match get_request() {
            UserRequest::Move(c) => match puzzle.update_cell(&c) {
                Ok(update) => println!("update: {}", update),
                Err(msg) => println!("unable to make move {} -> {} ", c, msg),
            },
            UserRequest::UndoMove => println!("TODO: implement undo"),
            UserRequest::Help => println!("TODO: implement help."),
            UserRequest::Quit => break,
            _ => (), // ignore the rest
        }

        if puzzle.is_completed() {
            println!("Congrats! You've won!");
            break;
        }
    }
}
