use std::collections::HashMap;
use std::fmt;

pub trait Repository<T: Clone> {
    fn find_by_id(&self, id: &str) -> Option<&T>;
    fn save(&mut self, id: String, item: T);
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

pub struct InMemoryRepo<T: Clone> {
    items: HashMap<String, T>,
}

impl<T: Clone> InMemoryRepo<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<T: Clone> Repository<T> for InMemoryRepo<T> {
    fn find_by_id(&self, id: &str) -> Option<&T> {
        self.items.get(id)
    }

    fn save(&mut self, id: String, item: T) {
        self.items.insert(id, item);
    }
}

pub type UserRepo = InMemoryRepo<User>;

pub enum Status {
    Active,
    Inactive,
    Suspended(String),
}

const MAX_RETRIES: u32 = 3;

fn create_user(name: &str, email: &str) -> User {
    User {
        name: name.to_string(),
        email: email.to_string(),
    }
}
