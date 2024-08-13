// ANCHOR: new-imports
pub use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
// ANCHOR_END: new-imports
pub use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
    Frame,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

pub use std::io::{self, stdout, Stdout};



mod tui;
pub use tui::*;

mod app;
pub use app::*;
