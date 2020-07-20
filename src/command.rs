use std::fmt;

use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::Resizable;
use cursive::views::{EditView, LinearLayout, TextView};
use cursive::Cursive;

use crate::{app::App, CONFIGURATION};

pub fn open_command_window(s: &mut Cursive) {
    let command_window = EditView::new()
        .filler(" ")
        .on_submit(call_on_app)
        .style(ColorStyle::new(
            Color::Dark(BaseColor::Black),
            Color::Dark(BaseColor::White),
        ))
        .fixed_width(CONFIGURATION.view_width * CONFIGURATION.grid_width);
    s.call_on_name("Frame", |view: &mut LinearLayout| {
        let mut commandline = LinearLayout::horizontal()
            .child(TextView::new(":"))
            .child(command_window);
        commandline.set_focus_index(1);
        view.add_child(commandline);
        view.set_focus_index(1);
    });
}

fn call_on_app(s: &mut Cursive, input: &str) {
    // things to do after recieving the command
    // 1. parse the command
    // 2. clean existing command messages
    // 3. remove the command window
    // 4. handle quit command
    s.call_on_name("Main", |view: &mut App| {
        let cmd = Command::from_string(input);
        view.clear_message();
        view.parse_command(cmd);
    });
    s.call_on_name("Frame", |view: &mut LinearLayout| {
        view.set_focus_index(0);
        view.remove_child(view.get_focus_index());
    });

    // special command that requires access to
    // our main cursive object, has to be parsed again
    // here
    // TODO: fix this somehow
    if let Ok(Command::Quit) = Command::from_string(input) {
        s.quit();
    }
}

#[derive(PartialEq)]
pub enum Command {
    Add(String, Option<u32>, bool),
    MonthPrev,
    MonthNext,
    Delete(String),
    TrackUp(String),
    TrackDown(String),
    SetGoal(u32),
    SetName(String),
    Quit,
    Blank,
}

#[derive(Debug)]
pub enum CommandLineError {
    InvalidCommand(String),
    InvalidArg(u32), // position
    NotEnoughArgs(String, u32),
}

impl std::error::Error for CommandLineError {}

impl fmt::Display for CommandLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandLineError::InvalidCommand(s) => write!(f, "Invalid command: `{}`", s),
            CommandLineError::InvalidArg(p) => write!(f, "Invalid argument at position {}", p),
            CommandLineError::NotEnoughArgs(s, n) => {
                write!(f, "Command `{}` requires atleast {} argument(s)!", s, n)
            }
        }
    }
}

type Result<T> = std::result::Result<T, CommandLineError>;

impl Command {
    pub fn from_string<P: AsRef<str>>(input: P) -> Result<Command> {
        let mut strings: Vec<&str> = input.as_ref().trim().split(' ').collect();
        if strings.is_empty() {
            return Ok(Command::Blank);
        }

        let first = strings.first().unwrap().to_string();
        let mut args: Vec<String> = strings.iter_mut().skip(1).map(|s| s.to_string()).collect();
        let mut _add = |auto: bool, first: String| {
            if args.is_empty() {
                return Err(CommandLineError::NotEnoughArgs(first, 1));
            }
            let goal = args
                .get(1)
                .map(|x| {
                    x.parse::<u32>()
                        .map_err(|_| CommandLineError::InvalidArg(2))
                })
                .transpose()?;
            return Ok(Command::Add(
                args.get_mut(0).unwrap().to_string(),
                goal,
                auto,
            ));
        };

        match first.as_ref() {
            "add" | "a" => _add(false, first),
            "add-auto" | "aa" => _add(true, first),
            "delete" | "d" => {
                if args.is_empty() {
                    return Err(CommandLineError::NotEnoughArgs(first, 1));
                }
                return Ok(Command::Delete(args[0].to_string()));
            }
            "track-up" | "tup" => {
                if args.is_empty() {
                    return Err(CommandLineError::NotEnoughArgs(first, 1));
                }
                return Ok(Command::TrackUp(args[0].to_string()));
            }
            "track-down" | "tdown" => {
                if args.is_empty() {
                    return Err(CommandLineError::NotEnoughArgs(first, 1));
                }
                return Ok(Command::TrackDown(args[0].to_string()));
            }
            "mprev" | "month-prev" => return Ok(Command::MonthPrev),
            "mnext" | "month-next" => return Ok(Command::MonthNext),
            "set-name" | "setn" => {
                if args.is_empty() {
                    return Err(CommandLineError::NotEnoughArgs(first, 1));
                }
                let name = &args[0];
                return Ok(Command::SetName(name.clone()));
            }
            "q" | "quit" => return Ok(Command::Quit),
            "" => return Ok(Command::Blank),
            s => return Err(CommandLineError::InvalidCommand(s.into())),
        }
    }
}
