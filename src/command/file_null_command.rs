use crate::command::command::Command;

pub struct FileNullCommand {}

impl FileNullCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for FileNullCommand {
    fn execute(&self) {}
}
