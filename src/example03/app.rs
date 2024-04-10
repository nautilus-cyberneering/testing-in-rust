use super::{
    user::User,
    user_repository::{self, UserRepository},
};

#[must_use]
pub fn run() -> String {
    app_factory().run()
}

fn app_factory() -> App {
    App::new(Box::new(user_repository::BTreeMapRepository::default()))
}

pub struct App {
    user_repository: Box<dyn UserRepository>,
}

impl App {
    #[must_use]
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn run(&mut self) -> String {
        let mut output = String::new();

        self.user_repository.add_user(User::new("Alice"));
        self.user_repository.add_user(User::new("Bob"));
        self.user_repository.add_user(User::new("Charlie"));

        for (i, user) in self.user_repository.get_all_users().iter().enumerate() {
            output.push_str(&format!("User {i}: {user:?}\n"));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::example03::{
        app::{app_factory, App},
        user::User,
        user_repository::UserRepository,
    };

    #[derive(Default)]
    pub struct VecRepository {
        users: Vec<User>,
    }

    impl UserRepository for VecRepository {
        fn add_user(&mut self, user: User) {
            self.users.push(user);
        }

        fn get_user(&self, name: &str) -> Option<&User> {
            self.users.iter().find(|u| u.name == name)
        }

        fn get_all_users(&self) -> Vec<&User> {
            self.users.iter().collect()
        }
    }

    #[test]
    fn test_using_the_real_repository() {
        let mut app = app_factory();

        let output = app.run();

        assert_eq!(output, "User 0: User { name: \"Alice\" }\nUser 1: User { name: \"Bob\" }\nUser 2: User { name: \"Charlie\" }\n");
    }

    #[test]
    fn test_using_the_repository_mock() {
        let mut app = App::new(Box::new(VecRepository::default()));

        let output = app.run();

        assert_eq!(output, "User 0: User { name: \"Alice\" }\nUser 1: User { name: \"Bob\" }\nUser 2: User { name: \"Charlie\" }\n");
    }
}
