use crate::model::user::{NewUser, User};
use crate::database::PostgresConnectionPool;
use crate::model::errors::{ApiResult, ApiError};
use diesel::{PgConnection, RunQueryDsl};
use diesel::prelude::*;

pub trait UserRepositoryTrait {
    fn add_new_user(&self, user: NewUser) -> ApiResult<User>;
    fn update_user(&self, user: User) -> ApiResult<String>;
    fn get_user(&self, user_id: i32) -> ApiResult<User>;
    fn get_all_users(&self) -> ApiResult<Vec<User>>;
    fn delete_user(&self, user_id: i32) -> ApiResult<String>;
}

pub struct UserRepository {
    database: PostgresConnectionPool,
}

impl UserRepository {

    pub fn new(database: PostgresConnectionPool) -> UserRepository {
        UserRepository { database }
    }
}

impl Clone for UserRepository {
    fn clone(&self) -> Self {
        UserRepository {
            database: self.database.clone()
        }
    }
}

impl UserRepositoryTrait for UserRepository {

    fn add_new_user(&self, user: NewUser) -> ApiResult<User> {
        use crate::schema::users::dsl::*;

        let db_conection: &PgConnection = &self.database.get().unwrap();

        let inserted_user = diesel::insert_into(users)
            .values(&user)
            .get_result(db_conection)?;

        Ok(inserted_user)
    }

    fn update_user(&self, user: User) -> ApiResult<String> {
        use crate::schema::users::dsl::*;

        let db_conection: &PgConnection = &self.database.get().unwrap();

        let target = users.filter(id.eq(user.id));
        diesel::update(target)
            .set(&user)
            .execute(db_conection)?;

        Ok("OK".to_string())
    }

    fn get_user(&self, user_id: i32) -> ApiResult<User> {
        use crate::schema::users::dsl::*;

        let db_conection: &PgConnection = &self.database.get().unwrap();

        let user = users.find(user_id)
            .first::<User>(db_conection)?;

        Ok(user)
    }

    fn get_all_users(&self) -> ApiResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        let db_conection: &PgConnection = &self.database.get().unwrap();
        let results = users.load::<User>(db_conection)?;

        Ok(results)
    }

    fn delete_user(&self, user_id: i32) -> ApiResult<String> {
        use crate::schema::users::dsl::*;

        let db_conection: &PgConnection = &self.database.get().unwrap();
        let result = diesel::delete(users.filter(id.eq(user_id)))
            .execute(db_conection)?;

        if result == 0 {
            Err(ApiError::new_error(1, String::from("User to be deleted not found!"), false))
        } else {
            Ok("OK".to_string())
        }
    }
}