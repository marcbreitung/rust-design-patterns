use crate::domain::model::document::Document;
use crate::command::command::Command;
use crate::domain;

pub struct FileOpenCommand<'a> {
    file: &'a Document
}

impl<'a> FileOpenCommand<'a> {
    pub fn new(file: &'a domain::model::document::Document) -> Self {
        Self {
            file
        }
    }
}

impl<'a> Command for FileOpenCommand<'a> {
    fn execute(&self) {
        self.file.open();
    }
}
