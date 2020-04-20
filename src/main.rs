use crate::command::file_close_command::FileCloseCommand;
use crate::command::file_delete_command::FileDeleteCommand;
use crate::command::file_open_command::FileOpenCommand;
use crate::command::file_save_command::FileSaveCommand;
use crate::command::menu::Menu;
use crate::domain::model::document::Document;

mod command;
mod domain;

fn main() {
    command_pattern();
}

fn command_pattern() {
    let file = Document::new(String::from("info.csv"), String::from("Name, Vorname\nBreitung, Marc\n"));
    let mut menu = Menu::new();

    menu.set_command(Box::new(FileOpenCommand::new(&file)));
    menu.click();

    menu.set_command(Box::new(FileCloseCommand::new(&file)));
    menu.click();

    menu.set_command(Box::new(FileSaveCommand::new(&file)));
    menu.click();

    menu.set_command(Box::new(FileDeleteCommand::new(&file)));
    menu.click();
}