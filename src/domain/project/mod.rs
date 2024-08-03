pub mod project_repository;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    id: Option<i32>,
    name: String,
    description: String
}

impl Project {

    pub fn new(id: Option<i32>, name: String, description: String) -> Self {
        Project {
            id,
            name,
            description
        }
    }

    pub fn to_string(&self) -> String {
        format!("id: {:?}\nname: {}\ndescription: {}", self.id, self.name, self.description)
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_description(&self) -> String {
        self.description.to_string()
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}