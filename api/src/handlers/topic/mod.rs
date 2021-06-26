use api_db::{Db, Model, Id};
use crate::util::respond;
use api_common::models::topic::{
    ScoreRequest, TopicVote, Category, TopicCategory, Topic
};
use actix_web::{HttpResponse, Responder, web::{Json, Path, Data, HttpRequest,  ServiceConfig, self}};


pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(add_topic))
        )

        .service(web::scope("/name/{topic}")
            .service(web::resource("")
                .route(web::get().to(get_by_name))
                .route(web::post().to(new_topic_name))
                .route(web::delete().to(delete_by_name))
            )
        )

        .service(web::scope("/category")
            .service(web::resource("")
                .route(web::get().to(get_all_categories))
                .route(web::post().to(add_category))
            )
            .service(web::scope("/{category_id}")
                .route("/score", web::get().to(get_topic_category_score))
                .route("/score", web::post().to(add_topic_category_score))
                .route("/score", web::put().to(update_topic_category_score))
                .route("", web::get().to(get_category_by_id))
                .route("", web::delete().to(del_category_by_id))
            )
        )

        .service(web::scope("/vote")
            .service(web::resource("")
                .route(web::get().to(get_all_topic_votes))
                .route(web::post().to(add_topic_vote))
            )
            .service(web::resource("/{vote_id}")
                .route(web::get().to(get_topic_vote_by_id))
                .route(web::delete().to(del_topic_vote_by_id))
            )
        )

        .service(web::scope("/{topic_id}")
            .service(web::resource("")
                .route(web::get().to(get_by_id))
                .route(web::delete().to(delete_by_id))
            )
            .service(web::scope("/category")
                .route("", web::get().to(get_topic_categories))
                .route("", web::post().to(add_topic_category_id))
            )
            .service(web::scope("/vote")
                .route("", web::get().to(get_topic_votes))
                .route("", web::post().to(add_topic_vote_by_id))
            )
        )

        ;
}

pub mod category {

}

pub async fn get_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Topic::get(&db.pool, id.into_inner()).await {
        Ok(Some(topic)) => respond::ok(topic),
        Ok(None) => respond::not_found("No topics"),
        Err(e) => respond::err(e),
    }
}
pub async fn get_by_name(db: Data<Db>, topic: Path<String>) -> impl Responder {
    match Topic::get_by_name(&db.pool, topic.into_inner()).await {
        Ok(Some(topic)) => respond::ok(topic),
        Ok(None) => respond::not_found("No topics"),
        Err(e) => respond::err(e),
    }
}
pub async fn delete_by_name(db: Data<Db>, topic: Path<String>) -> impl Responder {
    match Topic::delete_by_name(&db.pool, topic.into_inner()).await {
        Ok(Some(id)) => respond::ok(id),
        Ok(None) => respond::not_found("No topics"),
        Err(e) => respond::err(e),
    }
}
pub async fn delete_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Topic::delete(&db.pool, id.into_inner()).await {
        Ok(Some(id)) => respond::ok(id),
        Ok(None) => respond::not_found("No topics"),
        Err(e) => respond::err(e),
    }
}
pub async fn add_topic(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    match topic.into_inner().insert(&db.pool).await {
        Ok(topic) => respond::ok(topic),
        Err(e) => respond::err(e),
    }
}
pub async fn get_topic_categories(db: Data<Db>, topic_id: Path<Id>) -> impl Responder {
    match TopicCategory::linked_to_topic(&db.pool, topic_id.into_inner()).await {
        Ok(topic) => respond::ok(topic),
        Err(e) => respond::err(e),
    }
}
pub async fn get_topic_votes(db: Data<Db>, topic_id: Path<Id>) -> impl Responder {
    match TopicVote::linked_to_topic(&db.pool, topic_id.into_inner()).await {
        Ok(topic) => respond::ok(topic),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_topic_votes(db: Data<Db>) -> impl Responder {
    match TopicVote::get_all(&db.pool).await {
        Ok(topics) => respond::ok(topics),
        Err(e) => respond::err(e),
    }
}
pub async fn get_topic_vote_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match TopicVote::get(&db.pool, id.into_inner()).await {
        Ok(topic) => respond::ok(topic),
        Err(e) => respond::err(e),
    }
}
pub async fn get_category_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Category::get(&db.pool, id.into_inner()).await {
        Ok(cat) => respond::ok(cat),
        Err(e) => respond::err(e),
    }
}
pub async fn del_topic_vote_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match TopicVote::delete(&db.pool, id.into_inner()).await {
        Ok(topic) => respond::ok(topic),
        Err(e) => respond::err(e),
    }
}

pub async fn get_topic_category_score(db: Data<Db>, path: Path<(Id, Id)>) -> impl Responder {
    let (tid, cid) = path.into_inner();
    match TopicCategory::get_scores_between(&db.pool, tid, cid).await {
        Ok(c) => respond::ok(c),
        Err(e) => respond::err(e),
    }
}

pub async fn update_topic_category_score(db: Data<Db>, path: Path<(Id, Id)>, score: web::Query<ScoreRequest>) -> impl Responder {
    let (tid, cid) = path.into_inner();
    match TopicCategory::update_score(&db.pool, score.clone().into_inner().user_id, tid, cid, score.clone().into_inner().score).await {
        Ok(Some(c)) => respond::ok(c),
        Ok(None) => respond::not_found("No topics"),
        Err(e) => respond::err(e),
    }
}

pub async fn del_category_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Category::delete_by_id(&db.pool, id.into_inner()).await {
        Ok(cat) => respond::ok(cat),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_categories(db: Data<Db>) -> impl Responder {
    match Category::get_all(&db.pool).await {
        Ok(cat) => respond::ok(cat),
        Err(e) => respond::err(e),
    }
}
pub async fn add_category(db: Data<Db>, topic_id: Path<Id>, cat: Json<Category>, score: Json<ScoreRequest>) -> impl Responder {
    match cat.into_inner().insert(&db.pool).await {
        Ok(cat) => {
            let t_c = TopicCategory::new(score.clone().user_id, cat.id, topic_id.into_inner(), Some(score.score));
            match t_c.insert(&db.pool).await {
                Ok(topic_cat) => respond::ok(topic_cat),
                Err(e) => respond::err(e),
            }
        },
        Err(e) => respond::err(e),
    }
}
pub async fn add_topic_vote(db: Data<Db>, vote: Json<TopicVote>) -> impl Responder {
    match vote.into_inner().insert(&db.pool).await {
        Ok(vote) => respond::ok(vote),
        Err(e) => respond::err(e),
    }
}

pub async fn add_topic_vote_by_id(db: Data<Db>, vote: Json<TopicVote>) -> impl Responder {
    match vote.into_inner().insert(&db.pool).await {
        Ok(vote) => respond::ok(vote),
        Err(e) => respond::err(e),
    }
}

pub async fn add_topic_category_id(db: Data<Db>, topic_id: Path<Id>, category: Json<Category>, score: web::Query<ScoreRequest>) -> impl Responder {
    match category.into_inner().insert(&db.pool).await {
        Ok(c) => {
            let topic_cat = TopicCategory::new(score.clone().into_inner().user_id, c.id, topic_id.clone(), Some(score.score));
            match topic_cat.insert(&db.pool).await {
                Ok(tc) => respond::ok(tc),
                Err(e) => respond::err(e),
            }
        },
        Err(e) => respond::err(e),
    }
}

pub async fn add_topic_category_score(db: Data<Db>, path: Path<(Id, Id)>,score: web::Query<ScoreRequest>) -> impl Responder {
    let (topic_id, category_id) = path.into_inner();
    match TopicCategory::update_score(&db.pool, score.clone().into_inner().user_id, topic_id, category_id, score.score).await {
        Ok(Some(topic_category)) => respond::ok(topic_category),
        Ok(None) => respond::not_found("Topic not found"),
        Err(e) => respond::err(e),
    }
}

// #[post("/{topic}")]
pub async fn new_topic_name(db: Data<Db>, topic: Path<String>) -> impl Responder {
    let res: Id = sqlx::query_scalar("INSERT INTO topics (name) VALUES $1 returning id")
        .bind(topic.into_inner())
        .fetch_one(&db.pool).await.unwrap();
    respond::ok(res)
}

// #[get("/")]
pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Topic::get_all(&db.pool).await {
        Ok(topic) => respond::ok(topic),
        Err(e) => respond::internal_error().body(format!("{}", e))
    }
}

// #[get("/id/{topic_id}")]
pub async fn topic_id() -> impl Responder {
    "TOPIC ID".to_string()
}

