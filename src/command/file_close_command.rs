use crate::domain::model::document::Document;
use crate::command::command::Command;
use crate::domain;

pub struct FileCloseCommand<'a> {
    file: &'a Document
}

impl<'a> FileCloseCommand<'a> {
    pub fn new(file: &'a domain::model::document::Document) -> Self {
        Self {
            file
        }
    }
}

impl Command for FileCloseCommand<'_> {
    fn execute(&self) {
        self.file.close();
    }
}
