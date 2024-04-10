use super::user::User;

pub trait UserRepository {
    fn add_user(&mut self, user: User);
    fn get_user(&self, name: &str) -> Option<&User>;
    fn get_all_users(&self) -> Vec<&User>;
}

#[derive(Default)]
pub struct BTreeMapRepository {
    users: std::collections::BTreeMap<String, User>,
}

impl UserRepository for BTreeMapRepository {
    fn add_user(&mut self, user: User) {
        self.users.insert(user.name.clone(), user);
    }

    fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }

    fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}
