use actix_web::{web};
use actix_web::web::{Path};

use crate::service::user_service::{UserServiceTrait, UserService};
use crate::model::user::{NewUser, AddUserRequest, User, UpdateUserRequest};
use crate::handler::utils::{RestApiResult, to_json, validate_request};

pub fn add_new_user(user_service: web::Data<UserService>, user: web::Json<AddUserRequest>) -> RestApiResult<User> {
    let validation_errors = validate_request(&*user);
    if validation_errors.is_some() {
        return to_json(Result::Err(validation_errors.unwrap()))
    }
    let new_user = NewUser {
        first_name: &user.first_name[..],
        last_name: &user.last_name[..],
        email: &user.email[..],
    };
    let added_user = user_service.add_new_user(new_user);
    to_json(added_user)
}

pub fn get_all_users(user_service: web::Data<UserService>) -> RestApiResult<Vec<User>> {
    let users = user_service.get_all_users();
    to_json(users)
}

pub fn get_user(user_service: web::Data<UserService>, user_id: Path<i32>) -> RestApiResult<User> {
    let user = user_service.get_user(*user_id);
    to_json(user)
}

pub fn update_user(user_service: web::Data<UserService>, user: web::Json<UpdateUserRequest>) -> RestApiResult<String> {
    let validation_errors = validate_request(&*user);
    if validation_errors.is_some() {
        return to_json(Result::Err(validation_errors.unwrap()))
    }
    let updated_user = User {
        id: user.id,
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
    };
    to_json(user_service.update_user(updated_user))
}

pub fn delete_user(user_service: web::Data<UserService>, user_id: Path<i32>) -> RestApiResult<String> {
    to_json(user_service.delete_user(*user_id))
}