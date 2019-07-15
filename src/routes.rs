use actix_web::{web};
use crate::handler::health::health;
use crate::handler::user_handlers::{add_new_user, get_all_users, get_user, delete_user, update_user};
use crate::database::PostgresConnectionPool;
use crate::service::user_service::UserService;

pub fn routes(cfg: &mut web::ServiceConfig, database: PostgresConnectionPool) {
    let user_service = UserService::new(database);

    cfg.service( // create separate .service per scope, and pass corresponding Service implementation in shared state
        web::scope("/users")
            .data(user_service)
            .route("", web::get().to(get_all_users))
            .route("", web::put().to(add_new_user))
            .route("", web::post().to(update_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::delete().to(delete_user))
    )
//        .service(
//        // create separate .service per scope, and pass corresponding Service implementation in shared state
//        web::scope("/entity2")
//            .data(user_service)
//            .route("", web::get().to(get_all_entity2))
//            .route("", web::put().to(add_new_entity2))
//            .route("", web::post().to(update_entity2))
//            .route("/{id}", web::get().to(get_entity2))
//            .route("/{id}", web::delete().to(delete_entity2)
//    )

        .route("/", web::get().to(health));
}