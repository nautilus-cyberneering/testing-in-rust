use super::{
    user::User,
    user_repository::{BTreeMapRepository, UserRepository},
};

#[must_use]
pub fn run() -> String {
    app_factory().run()
}

fn app_factory() -> App<BTreeMapRepository> {
    App::new(BTreeMapRepository::default())
}

pub struct App<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> App<T> {
    #[must_use]
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub fn run(&mut self) -> String {
        let mut output = String::new();

        self.user_repository.add_user(User::new("Alice"));
        self.user_repository.add_user(User::new("Bob"));
        self.user_repository.add_user(User::new("Charlie"));

        /*for i in 1..10_000_000 {
            self.user_repository
                .add_user(User::new(&format!("User {i}")));
        }*/

        for (i, user) in self.user_repository.get_all_users().iter().enumerate() {
            output.push_str(&format!("User {i}: {user:?}\n"));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::example04::{
        app::{app_factory, App},
        user_repository::VecRepository,
    };

    #[test]
    fn test_using_the_real_repository() {
        let mut app = app_factory();

        let output = app.run();

        assert_eq!(
            output,
            "User 0: User { name: \"Alice\" }\nUser 1: User { name: \"Bob\" }\nUser 2: User { name: \"Charlie\" }\n"
        );
    }

    #[test]
    fn test_using_the_repository_mock() {
        let mut app = App::new(VecRepository::default());

        let output = app.run();

        assert_eq!(
            output,
            "User 0: User { name: \"Alice\" }\nUser 1: User { name: \"Bob\" }\nUser 2: User { name: \"Charlie\" }\n"
        );
    }
}
