use crate::domain::model::actor::Actor;

pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

pub struct WalkCommand<'a> {
    actor: &'a mut Actor,
    x: u32,
    y: u32,
    x_before: u32,
    y_before: u32,
}

impl<'a> WalkCommand<'a> {
    pub fn new(actor: &'a mut Actor, x: u32, y: u32) -> Self {
        Self {
            actor,
            x,
            y,
            x_before: 0,
            y_before: 0,
        }
    }
}

impl<'a> Command for WalkCommand<'a> {
    fn execute(&mut self) {
        self.x_before = self.actor.x;
        self.y_before = self.actor.y;
        self.actor.walk_to(self.x, self.y);
    }

    fn undo(&mut self) {
        self.actor.walk_to(self.x_before, self.y_before);
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::model::actor::Actor;
    use crate::command::command::{WalkCommand, Command};

    #[test]
    fn execute() {
        let mut actor = Actor::new(String::from("Actor"), 10);
        let mut walk_command = WalkCommand::new(&mut actor, 25, 50);
        walk_command.execute();
        assert_eq!(actor.get_position(), (25, 50));
    }

    #[test]
    fn undo() {
        let mut actor = Actor::new(String::from("Actor"), 10);
        let mut walk_command = WalkCommand::new(&mut actor, 25, 50);
        walk_command.execute();
        walk_command.undo();
        assert_eq!(actor.get_position(), (0, 0));
    }
}