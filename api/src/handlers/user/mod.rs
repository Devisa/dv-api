pub mod verification;
pub mod level;
pub mod badge;
pub mod profile;
pub mod account;
pub mod session;
pub mod credentials;
pub mod link;

use api_db::{Model, Id, Db};
use crate::util::respond;
use api_common::models::User;
use actix_web::{
    HttpRequest, Responder, http::StatusCode,
    web::{
        self, ServiceConfig, Path, Data, Form, Json,
    }
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/account").configure(account::routes))
        .service(web::scope("/verification").configure(verification::routes))
        .service(web::scope("/session").configure(session::routes))
        .service(web::scope("/credentials").configure(credentials::routes))
        .service(web::scope("/profile").configure(profile::routes))
        .service(web::scope("/user").route("", web::get().to(get_all_user_user_links)))
        .service(web::scope("/link").configure(link::routes))

        .service(web::scope("/{user_id}").configure(individual_user_ops))
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_user))
            .route(web::put().to(generate_fake_users))
        )
        .service(web::scope("/gen").configure(gen_user_routes))
        .service(web::scope("/username/{username}").configure(by_username));
}
pub fn individual_user_ops(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("")
            .route("", web::get().to(get_by_id))
            .route("", web::post().to(update_by_id))
            .route("", web::delete().to(delete_by_id))
            .service(web::scope("/link")
                .configure(link::user_routes)
            )
        )
        .service(web::resource("/session")
            .route(web::get().to(get_user_sessions))
        )
        .service(web::resource("/account")
            .route(web::get().to(get_user_accounts))
        )
        .service(web::resource("/profile")
            .route(web::get().to(get_user_profile))
            .route(web::post().to(update_user_profile))
        )
        .service(web::resource("/credentials")
            .route(web::get().to(get_user_credentials))
            .route(web::post().to(update_user_credentials))
        )
        .service(web::resource("/item")
            .route(web::get().to(get_user_items))
        )
        .service(web::resource("/field")
            .route(web::get().to(get_user_fields))
        )
        .service(web::resource("/level")
            .route(web::get().to(get_user_level))
        )
        .service(web::resource("/badge")
            .route(web::get().to(get_user_badges))
        )
        .service(web::scope("/group")
            .route("", web::get().to(get_user_created_groups))
            .route("/member", web::get().to(get_user_member_groups))
        )
        .service(web::resource("/post")
            .route(web::get().to(get_user_posts))
        )
        .service(web::scope("/user")   // "connects" -- l;inks, between users
            .route("", web::get().to(get_user_user_links))
            .route("/{user_id}", web::post().to(link_user_to_user))
        )
        .service(web::resource("/connects")   // "connects" -- l;inks, between users
            .route(web::get().to(get_user_connects))
        );
}

pub fn gen_user_routes(cfg: &mut ServiceConfig) {
    cfg
    .route("", web::get().to(gen_user))
    .route("", web::post().to(insert_gen_user))
    .service(web::resource("/{gen_no}")
        .route(web::get().to(gen_n_users))
        .route(web::post().to(insert_n_gen_users))
    );
}
pub fn by_username(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_username))
            .route(web::post().to(update_by_username))
            .route(web::delete().to(delete_by_username))
        );

}

pub async fn generate_fake_users(req: HttpRequest, db: Data<Db>) -> impl Responder {
    "OK".to_string()

}
pub async fn get_all(db: Data<Db>) -> impl Responder {
    match User::get_all(&db.pool).await {
        Ok(users) => respond::ok(users),
        Err(e) => respond::err(e),
    }
}

/// POST /user : Create a new user with (at least) name and unverified email.
///     Stage 1 of signup process. -> 2. Create credentials
pub async fn new_user(user: Json<User>, db: Data<Db>) -> impl Responder {
    match user.into_inner().insert(&db.pool).await {
        Ok(user) => respond::ok(user),
        Err(e) => respond::err(e),
    }
}

pub async fn update_by_id(id: Path<Id>, db: Data<Db>) -> impl Responder {
    match User::get(&db.pool, id.into_inner()).await {
        Ok(Some(user)) => respond::found(user),
        Ok(None) => respond::not_found("NO USER FOUND"),
        Err(e) => respond::err(e)
    }
}

pub async fn get_by_id(id: Path<Id>, db: Data<Db>) -> impl Responder {
    match User::get(&db.pool, id.into_inner()).await {
        Ok(Some(user)) => respond::found(user),
        Ok(None) => respond::not_found("NO USER FOUND"),
        Err(e) => respond::err(e),
    }
}

pub async fn get_by_username(username: Path<String>, db: Data<Db>) -> impl Responder {
    match User::get_by_username(&db.pool, &username).await {
        Ok(Some(user)) => respond::found(user),
        Ok(None) => respond::not_found("NO USER FOUND"),
        Err(e) => respond::err(e),
    }
}

pub async fn update_by_username(username: Path<String>, db: Data<Db>) -> impl Responder {
    let user = User::get_by_username(&db.pool, &username).await
        .expect("Could not fetch user");
    match user {
        Some(user) => match User::get(&db.pool, user.id).await {
            Ok(Some(user)) => respond::found(user),
            Ok(None) => respond::not_found("NO USER FOUND"),
            Err(e) => respond::err(e),
        },
        None => respond::not_found("User not found"),
    }
}

pub async fn delete_by_username(username: Path<String>, db: Data<Db>) -> impl Responder {
    let user = User::get_by_username(&db.pool, &username).await
        .expect("Could not fetch user");
    if let Some(user) = user {
        match User::delete(&db.pool, user.id).await {
            Ok(Some(user)) => respond::found(user),
            Ok(None) => respond::not_found("NO USER FOUND"),
            Err(e) => respond::err(e),
        }
    } else {
        respond::not_found("User not found")
    }
}

pub async fn delete_by_id(id: Path<Id>, db: Data<Db>) -> impl Responder {
    match User::delete(&db.pool, id.into_inner()).await {
        Ok(Some(user)) => respond::found(user),
        Ok(None) => respond::not_found("NO USER FOUND"),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_created_groups(id: Path<Id>, db: Data<Db>) -> impl Responder {
    match User::get_items_created(&db.pool, id.into_inner()).await {
        Ok(groups) => respond::ok(groups),
        Err(e) => respond::err(e),
    }
}
//TODO unimplemented
pub async fn get_user_member_groups(id: Path<Id>, db: Data<Db>) -> impl Responder {
    match User::get_items_created(&db.pool, id.into_inner()).await {
        Ok(groups) => respond::ok(groups),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_sessions(id: Path<Id>, db: Data<Db>) -> impl Responder {
    match User::get_sessions(&db.pool, id.into_inner()).await {
        Ok(sess) => respond::found(sess),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_accounts(id: Path<Id>, db: Data<Db>) -> impl Responder {
    match User::get_accounts(&db.pool, id.into_inner()).await {
        Ok(accts) => respond::found(accts),
        Err(e) => respond::err(e),
    }
}

pub async fn update_user_credentials() -> impl Responder {
    "".to_string()
}
pub async fn update_user_profile() -> impl Responder {
    "".to_string()
}
pub async fn get_user_credentials(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match User::get_credentials(&db.pool, id.into_inner()).await {
        Ok(c) => respond::found(c),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_profile(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match User::get_profile(&db.pool, id.into_inner()).await {
        Ok(p) => respond::found(p),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_records(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match User::get_records_created(&db.pool, id.into_inner()).await {
        Ok(r) => respond::found(r),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_items(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match User::get_items_created(&db.pool, id.into_inner()).await {
        Ok(i) => respond::found(i),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_fields(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match User::get_fields_created(&db.pool, id.into_inner()).await {
        Ok(f) => respond::found(f),
        Err(e) => respond::err(e),
    }
}
// TODO unimplemented
pub async fn get_user_groups(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match User::get_groups_created(&db.pool, id.into_inner()).await {
        Ok(g) => respond::found(g),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_topics() -> impl Responder {
    "".to_string()
}
pub async fn get_user_posts() -> impl Responder {
    "".to_string()
}
pub async fn get_user_connects() -> impl Responder {
    "".to_string()
}
pub async fn get_user_user_links() -> impl Responder {
    "".to_string()
}
pub async fn link_user_to_user() -> impl Responder {
    "".to_string()
}
pub async fn get_all_user_user_links() -> impl Responder {
    "".to_string()
}
pub async fn get_user_level(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match User::get_level(&db.pool, user_id.into_inner()).await {
        Ok(u) => respond::ok(u),
        Err(e) => respond::err(e),
    }
}
pub async fn get_user_badges(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match User::get_badges(&db.pool, user_id.into_inner()).await {
        Ok(u) => respond::ok(u),
        Err(e) => respond::err(e),
    }
}
pub async fn gen_user() -> impl Responder {
    let user = User::gen();
    respond::ok(user)
}
pub async fn insert_gen_user(db: Data<Db>) -> impl Responder {
    let user = User::gen();
    match user.insert(&db.pool).await {
        Ok(u) => respond::ok(u),
        Err(e) => respond::err(e),
    }
}
pub async fn gen_n_users(db: Data<Db>, num: Path<u8>) -> impl Responder {
    let mut users: Vec<User> = Vec::new();
    for _ in 0..num.into_inner() {
        let user = User::gen();
        users.push(user);
    }
    respond::ok(users)
}
pub async fn insert_n_gen_users(db: Data<Db>, num: Path<u8>) -> impl Responder {
    let mut users: Vec<User> = Vec::new();
    for _ in 0..num.into_inner() {
        match User::gen().insert(&db.pool).await {
            Ok(u) => users.push(u),
            Err(e) => { return respond::err(e); }
        }
    }
    respond::ok(users)
}


// impl Responder for User {
//     fn respond_to(self, req: &HttpRequest) -> HttpResponse {
//         respond::ok().body("h")

//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use actix_web::{test::{self, TestRequest}, web, };

    #[actix_rt::test]
    async fn insert_two_non_unique_emails_fails() -> anyhow::Result<()> {
        Ok(())
    }

    #[actix_rt::test]
    async fn get_all_users_ok() -> anyhow::Result<()> {
        let db = db().await?;
        let u1 = add_user(&db, "user1", "user1@email.com").await?;
        let u2 = add_user(&db, "user2", "user2@email.com").await?;
        let u3 = add_user(&db, "user3", "user3@email.com").await?;
        let v = vec![u1, u2, u3];
        let req = TestRequest::get().uri("/user");
        /* let resp = get_all(Data::new(db)).await
            .respond_to(&req); */
        // assert!(resp.status().is_ok());
        Ok(())
    }

    #[actix_rt::test]
    async fn get_user_by_id_ok() -> anyhow::Result<()> {
        let db = db().await?;
        let srv = service("/user/{id}", web::get().to(get_all)).await;
        /* let add_user = (&db, "user1", "user1@email.com").await?;
        let req = TestRequest::get().uri("/user/1")
            .to_http_request(); */
        /* let resp = req
            .send_request(&srv).await; */
        /* let resp = get_by_id(Data::new(db), Path::new(1)).await;
        assert!(resp.status().is_ok()); */
        Ok(())
    }

    #[actix_rt::test]
    async fn add_user_ok() -> anyhow::Result<()> {
        let db = db().await?;
        /* let user = add_user("user1", "user1@email.com").await;
        let session = User::default().insert(&db.pool).await?; */
        // let srv = service("/user", web::post().to(new_user)).await;
        Ok(())
    }
}
