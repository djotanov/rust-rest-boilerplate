use crate::model::user::{NewUser, User};
use crate::repository::user_repository::{UserRepository};
use crate::database::PostgresConnectionPool;
use crate::model::errors::ApiResult;
use crate::repository::user_repository::UserRepositoryTrait;

pub trait UserServiceTrait {
    fn add_new_user(&self, user: NewUser) -> ApiResult<User>;
    fn update_user(&self, user: User) -> ApiResult<String>;
    fn get_user(&self, user_id: i32) -> ApiResult<User>;
    fn get_all_users(&self) -> ApiResult<Vec<User>>;
    fn delete_user(&self, user_id: i32) -> ApiResult<String>;
}

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(database: PostgresConnectionPool) -> UserService {
        let repository = UserRepository::new(database);

        UserService {
            repository
        }
    }
}

impl Clone for UserService {
    fn clone(&self) -> Self {
        UserService {
            repository: self.repository.clone()
        }
    }
}

impl UserServiceTrait for UserService {

    fn add_new_user(&self, user: NewUser) -> ApiResult<User> {
        self.repository.add_new_user(user)
    }

    fn update_user(&self, user: User) -> ApiResult<String> {
        self.repository.update_user(user)
    }

    fn get_user(&self, user_id: i32) -> ApiResult<User> {
        self.repository.get_user(user_id)
    }

    fn get_all_users(&self) -> ApiResult<Vec<User>> {
        self.repository.get_all_users()
    }

    fn delete_user(&self, user_id: i32) -> ApiResult<String> {
        self.repository.delete_user(user_id)
    }
}