use crate::domain::model::document::Document;
use crate::command::command::Command;
use crate::domain;

pub struct FileDeleteCommand<'a> {
    file: &'a Document
}

impl<'a> FileDeleteCommand<'a> {
    pub fn new(file: &'a domain::model::document::Document) -> Self {
        Self {
            file
        }
    }
}

impl Command for FileDeleteCommand<'_> {
    fn execute(&self) {
        self.file.delete();
    }
}

