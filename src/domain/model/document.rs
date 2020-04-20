pub struct Document {
    name: String,
    body: String,
}

impl Document {
    pub fn new(name: String, body: String) -> Self {
        Self {
            name,
            body,
        }
    }

    pub fn name(&self) {
        println!("The name is {}", self.name);
    }

    pub fn body(&self) {
        println!("The body is {}", self.body);
    }

    pub fn open(&self) {
        println!("Open file {}", self.name);
    }

    pub fn close(&self) {
        println!("Close file {}", self.name);
    }

    pub fn save(&self) {
        println!("Save file {}", self.name);
    }

    pub fn delete(&self) {
        println!("Delete file {}", self.name);
    }
}