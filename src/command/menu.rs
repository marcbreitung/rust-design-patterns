use crate::command::command::Command;
use crate::command::file_null_command::FileNullCommand;

pub struct Menu<'a> {
    command: Box<dyn Command + 'a>,
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        Menu { command: Box::new(FileNullCommand::new()) }
    }
    pub fn set_command(&mut self, cmd: Box<dyn Command + 'a>) {
        self.command = cmd;
    }
    pub fn click(&self) {
        self.command.execute();
    }
}
