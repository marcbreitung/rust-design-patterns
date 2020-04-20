use crate::domain::model::document::Document;
use crate::command::command::Command;
use crate::domain;

pub struct FileSaveCommand<'a> {
    file: &'a Document
}

impl<'a> FileSaveCommand<'a> {
    pub fn new(file: &'a domain::model::document::Document) -> Self {
        Self {
            file
        }
    }
}

impl Command for FileSaveCommand<'_> {
    fn execute(&self) {
        self.file.save();
    }
}

