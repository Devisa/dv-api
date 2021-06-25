
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, guard, web::{self, ServiceConfig}};
use async_graphql::{
    Context, Data, EmptyMutation, Object, Schema, Subscription,
    http::{
        playground_source, GraphQLPlaygroundConfig
    }
};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use futures::{stream, Stream};

pub fn routes(cfg: &mut ServiceConfig) {
    // let schema = Schema::new(QueryRoot, EmptyMutation, SubscriptionRoot);
    cfg;
        // .service(index);
        // .data(schema.clone())
        // .service(web::resource("/").guard(guard::Post()).to(index));
        /* .service(
            web::resource("/")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        )
        .service(web::resource("/").guard(guard::Get()).to(gql_playgound)); */
}

/* pub type DvSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub struct DvToken(String);

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<DvToken>().map(|token| token.0.as_str())
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
        if ctx.data::<DvToken>()?.0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(stream::once(async move { 10 }))
    }
}
 */
/* #[get("/")]
async fn index() -> impl Responder {
    "Helo!".to_string()
} */
/* async fn index(schema: web::Data<DvSchema>, req: HttpRequest, gql_request: Request) -> Response {
    let token = req
        .headers()
        .get("Token")
        .and_then(|value| value.to_str().map(|s| DvToken(s.to_string())).ok());
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

async fn index_ws(
    schema: web::Data<DvSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    WSSubscription::start_with_initializer(Schema::clone(&*schema), &req, payload, |value| async {
        #[derive(serde_derive::Deserialize)]
        struct Payload {
            token: String,
        }

        if let Ok(payload) = serde_json::from_value::<Payload>(value) {
            let mut data = Data::default();
            data.insert(DvToken(payload.token));
            Ok(data)
        } else {
            Err("Token is required".into())
        }
    })
}
 */
