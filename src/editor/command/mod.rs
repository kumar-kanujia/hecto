mod edit;
mod movecommand;
mod system;

pub use crate::{
  editor::command::{edit::Edit, movecommand::Move, system::System},
  prelude::*,
};

use crossterm::event::Event;

#[derive(Clone, Copy)]
pub enum Command {
  Move(Move),
  Edit(Edit),
  System(System),
}

// clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
#[allow(clippy::as_conversions)]
impl TryFrom<Event> for Command {
  type Error = String;

  fn try_from(event: Event) -> Result<Self, Self::Error> {
    match event {
      Event::Key(key_event) => Edit::try_from(key_event)
        // Try to convert key_event into an Edit event
        .map(Command::Edit)
        // If fail, try to convert key_event into a Move event
        .or_else(|_| Move::try_from(key_event).map(Command::Move))
        // If fail, try to convert key_event into a System event
        .or_else(|_| System::try_from(key_event).map(Command::System))
        // If fail, return an error
        .map_err(|_err| format!("Event not supported: {key_event:?}")),
      Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
        height: height_u16 as usize,
        width: width_u16 as usize,
      }))),
      _ => Err(format!("Event not supported: {event:?}")),
    }
  }
}
