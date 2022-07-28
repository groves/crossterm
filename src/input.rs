use std::fmt;
use crate::event::filter::{EventFilter};
use crate::{csi, Command, Result};
use crate::event::{Event, InternalEvent, read_internal};

pub fn read() -> Result<Input> {
    match read_internal(&EventFilter)? {
        InternalEvent::Event(event) => Ok(Input::Event(event)),
        InternalEvent::Input(input) => Ok(input),
        #[cfg(unix)]
        _ => unreachable!()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub enum Input {
    Event(Event),
    Paste(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableBracketedPaste;

impl Command for EnableBracketedPaste {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("?2004h"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableBracketedPaste;

impl Command for DisableBracketedPaste {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("?2004l"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

