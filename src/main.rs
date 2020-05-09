use crate::domain::model::actor::Actor;
use crate::command::command::{WalkCommand, Command};

mod command;
mod domain;

fn main() {
    let mut actor = Actor::new(String::from("Actor"), 10);
    let mut walk_command = WalkCommand::new(&mut actor, 25, 50);
    walk_command.execute();
    walk_command.undo();
    println!("{:?}", actor.get_position());
    println!("{:?}", actor);
}
