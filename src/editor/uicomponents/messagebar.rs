use crate::editor::{size::Size, terminal::Terminal, uicomponents::UIComponent};

use std::{
  io::Error,
  time::{Duration, Instant},
};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

struct Message {
  text: String,
  time: Instant,
}

impl Default for Message {
  fn default() -> Self {
    Self {
      text: String::new(),
      time: Instant::now(),
    }
  }
}

impl Message {
  fn is_expired(&self) -> bool {
    Instant::now().duration_since(self.time) > DEFAULT_DURATION
  }
}

#[derive(Default)]
pub struct MessageBar {
  current_message: Message,
  need_redraw: bool,
  // ensures we can properly hide expired messages
  // once a message expires we need to
  cleared_after_expiry: bool,
}

impl MessageBar {
  pub fn update_message(&mut self, new_message: &str) {
    self.current_message = Message {
      text: new_message.to_string(),
      time: Instant::now(),
    };
    self.cleared_after_expiry = false;
    self.set_needs_redraw(true);
  }
}

impl UIComponent for MessageBar {
  fn set_needs_redraw(&mut self, value: bool) {
    self.need_redraw = value;
  }

  fn needs_redraw(&self) -> bool {
    // Check if current message is cleared we send redraw ever time till it gets cleared
    let is_message_cleared = !self.cleared_after_expiry && self.current_message.is_expired();
    is_message_cleared || self.need_redraw
  }

  fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
    if self.current_message.is_expired() {
      // Upon expiration, we need to write out "" once to clear the message.
      // To avoid clearing more than necessary, we  keep track of the fact that we've already cleared the expired message once.
      self.cleared_after_expiry = true;
    }
    let message = if self.current_message.is_expired() {
      ""
    } else {
      &self.current_message.text
    };
    Terminal::print_row(origin_y, message)
  }

  fn set_size(&mut self, _: Size) {}
}
