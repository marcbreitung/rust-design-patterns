#[derive(Debug)]
pub struct Actor {
    name: String,
    strength: u32,
    pub x: u32,
    pub y: u32,
}

impl Actor {
    pub fn new(name: String, strength: u32) -> Self {
        Self {
            name,
            strength,
            x: 0,
            y: 0,
        }
    }

    pub fn walk_to(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::model::actor::Actor;

    #[test]
    fn can_move_to() {
        let mut actor = Actor::new(String::from("Marc"), 10);
        assert_eq!(actor.get_position(), (0, 0));

        actor.walk_to(10, 10);
        assert_eq!(actor.get_position(), (10, 10));
    }
}