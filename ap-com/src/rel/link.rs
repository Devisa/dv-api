//! This module handles trait implementations of Links.
//! Models (structs) which are linked, that is, have a many-to-many relation
//! and are thus stored in a relation database with join tables
//! (like ItemField) will implement LinkedTo<Other> individually,
//! and the join table (ItemField) will implement Linked.
use actix_web::{HttpResponse, Resource, Responder, Scope, web::{Data, Path, self, ServiceConfig}};
use crate::{models::Model, types::Id, util::respond, Db};
use sqlx::{
    FromRow, Postgres, postgres::{PgRow, PgPool},
};

/// TODO Think that this is the more appropriate way to generalize this,
///     not generics

/// Trait to implement functionality for struct linked to another by join table
///     This trait assumes it is the *left* in the link (for path purposes)
#[async_trait::async_trait]
pub trait LinkedTo<L>
where
    L: Model + for<'r> FromRow<'r, PgRow>,
    Self: Model + for<'r> FromRow<'r, PgRow>
{

    type LinkModel: Linked + Model + for<'r> FromRow<'r, PgRow> ;

    /// Get the path of LinkModel, given the struct for which the linked
    /// struct is implicitly the base route. So, <User as LinkedTo<Group>>::link_path
    /// would give the group as the base path, "/user/{id}/group"
    /// This way requests to the link path return the Self model type, i.e.
    /// the service_linked_to called at the link_path for <User as LinkedTo<Group>>
    /// would return users with group id from "/group/{id}"
    /// Example:
    ///     ```
    ///     let field_item = <Field as LinkedTo<Item>>::link_path();
    ///     //             = "/{id}/item"
    ///     ```
    fn path() -> String {
        let mut base = String::from("/{id}");
        base.push_str(<L as Model>::path().as_str());
        return base;
    }

    fn scope() -> Scope {
        web::scope(&<Self as LinkedTo<L>>::path())
            .configure(<Self as LinkedTo<L>>::routes)
            .service(web::resource("")
                .route(web::get().to(<Self as LinkedTo<L>>::service_get_all_linked_to))
                .route(web::delete().to(<Self as LinkedTo<L>>::service_delete_all_linked_to))
                // TODO add route to post new link given Json<L>
                // TODO add route to get all link models, not other linked type L, from id
            )
    }

    // TODO unimplemented -- meant to provide route handlers for managing explicit link
    // rows given one ID (self id). Other scope() will then provide interface to manage
    // structs of type L given self id
    fn linked_routes() -> Scope {
        web::scope("")
    }

    /// Meant to be implemented individually by link structs, contains
    /// all non-generalizable link API handlers
    fn routes(cfg: &mut ServiceConfig) {
        cfg;
    }

    async fn service_get_all_linked_to(db: Data<Db>, id: Path<Id>) -> actix_web::Result<HttpResponse> {
        match <Self as LinkedTo<L>>::get_all_linked_to(&db.pool, id.into_inner()).await {
            Ok(l_type) => Ok(respond::ok(l_type)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_delete_all_linked_to(db: Data<Db>, id: Path<Id>) -> actix_web::Result<HttpResponse> {
        match <Self as LinkedTo<L>>::delete_all_linked_to(&db.pool, id.into_inner()).await {
            Ok(l_model) => Ok(respond::ok(l_model)),
            Err(e) => Ok(respond::err(e)),
        }
    }

    /// Deletes all structs of type L linked to Self struct with id provided.
    /// ```
    /// // Deletes all groups for which user with user_id is a member
    /// let groups = <User as LinkedTo<Group>>::delete_all_linked_to(db, user_id).await?;
    /// // Deletes all items asssociated with record with record_id provided
    /// let items = <Record as LinkedTo<Item>>::delete_all_linked_to(db, record_id).await?;
    /// ```
    async fn delete_all_linked_to(db: &PgPool, this_id: Id) -> sqlx::Result<Vec<L>> {
        let query_str = format!("
            DELETE FROM {other}
            INNER JOIN {link} on {link}.{other_id_str} = {other}.id
            INNER JOIN {link} on {link}.{this_id_str} = {this}.id
            WHERE {this}.id = $1 RETURNING * ",
            other = <L as Model>::table(),
            link = <Self::LinkModel as Model>::table(),
            this = <Self as Model>::table(),
            this_id_str = <Self as Model>::id_str(),
            other_id_str = <L as Model>::id_str(),);
        let res = sqlx::query_as::<Postgres, L>(&query_str)
            .bind(this_id)
            .fetch_all(db).await?;
        Ok(res)

    }


    /// Get links to struct of the type for which this trait is being implemented.
    /// So if struct is User, then calling this on:
    /// ```
    /// // Returns the groups which have user (with id user_id) as a member
    /// let users: Vec<User> = <Group as LinkedTo<User>>::get_links(db, group_id).await?;
    /// let groups: Vec<Group> = <User as LinkedTo<Group>>::get_links(db, user_id).await?;
    /// // Returns the members (users) of the group with group_id
    /// let users: Vec<User> = <User as LinkedTo<Group>>::linked_to(db, group_id).await?;
    ///
    /// // Returns the items which have a link to a given field_id
    /// let items: Vec<Item> = <Item as LinkedTo<Field>>::linked_to(db, field_id).await?;
    /// ```
    async fn get_all_linked_to(db: &PgPool, this_id: Id) -> sqlx::Result<Vec<L>> {
        let res = sqlx::query_as::<Postgres, L>(&format!("
            SELECT * FROM {other}
            INNER JOIN {link} ON {other}.id = {link}.{other_id_str}
            INNER JOIN {this} ON {this}.id = {link}.{this_id_str}
            WHERE {this}.id = $1
            ",
            other = L::table(),
            link = Self::LinkModel::table(),
            this = Self::table(),
            this_id_str = Self::id_str(),
            other_id_str = L::id_str(),
            ))
            .bind(this_id)
            .fetch_all(db).await?;
        Ok(res)
    }
}

/// Linked model implementing routes and handlers for structs like ItemField
/// unlike the LinkedTo trait, **order matters**. If one wants to insert a link
/// between an Item and a Field, given both their ids, they must do so through
/// the service provided by the path <ItemField as Linked>::link_path()
/// = "/item/{id}/field/{id}". A link between two structs provides handlers at the following:
///     - <Item as LinkedTo<Field>>::link_path() = "/{id}/item" (a service of "/field")
///     - <Field as LinkedTo<Item>>::link_path() = "/{id}/field" (a service of "/item")
///     - <Item as LinkedTo<Record>>::link_path() = "/{id}/item" (a service of "/record")
///     - <ItemField as Linked>::link_path() = "/{id}/field/{id}" (ONLY a service of "/item")
/// So the left struct in the link name ("ITEMfield", "RECORDitem") designates the base service
#[async_trait::async_trait]
pub trait Linked: Model + Default + Clone {

    // NOTE: Don't think its's necessary to have LinekdTo as type param
    //     or to have it at all really
    type Left: Model + LinkedTo<Self::Right>;
    type Right: Model + LinkedTo<Self::Left>;

    fn link_id(self) -> Option<Id>;

    fn left_id(self) -> Id {
        Id::gen()
    }
    fn right_id(self) -> Id {
        Id::gen()
    }

    /// Relative path if the left model is base path implicitly
    ///     Ex. For Left = Item, Right = Field, base_left_path()
    ///     returns "/{id}/field"
    fn right_base_path() -> String {
        let mut path = <Self::Left as LinkedTo<Self::Right>>::path();
        path.push_str("/{link_id}");
        return path;
    }

    /// This is THE base link path to access link-specific handlers
    ///     ex. <GroupUser as Linked>::links_between_path() = "/{id}/user/{link_id}"
    /// Intended to be implemented such that "group/" is the base path, as such:
    /// ```
    /// impl Model for Group {
    ///     fn path() -> String { String::from("/group") }
    ///     // Served at "/group"
    ///     fn model_routes(cfg: &mut ServieConfig) {
    ///         cfg
    ///             // Served at "/group/{id}/user"
    ///             .service(<Group as LinkedTo<User>>::linked_to_scope())
    ///             // Served at "/group/{id}/user/{link_id}"
    ///             .service(<GroupUser as Linked>::links_between_scope())
    ///
    ///     }
    /// }
    /// ```
    fn path() -> String {
        let mut path = <Self::Left as LinkedTo<Self::Right>>::path();
        path.push_str("/{link_id}");
        return path;
    }

    fn scope() -> actix_web::Scope {
        web::scope(&<Self as Linked>::path())
            .configure(<Self as Linked>::routes)
            .service(<Self as Model>::scope())
            .service(web::resource("")
                .route(web::get().to(<Self as Linked>::service_links_between))
                .route(web::post().to(<Self as Linked>::service_insert_link_between))
                .route(web::delete().to(<Self as Linked>::service_delete_links_between))
            )
    }

    /// Meant to be implemented for link-specific handlers and routes
    fn routes(cfg: &mut ServiceConfig) {
        cfg;
    }

    /// Default new function for join table consisting only of L, R ids + opt link_id
    /// Reimplement, or overwrite this function in the case of join table with other
    /// required fields
    fn new_basic(left_id: Id, right_id: Id, link_id: Option<Id>) -> Self {
        Self::default()
    }

    /// Default insert function for a join table consisting only of L, R ids + optional link_id
    ///     (and created, updated). Re-implement for join tables with extra required fields
    ///     ex. TopicCategory, etc.
    async fn insert_link_between(db: &PgPool, left_id: Id, right_id: Id, link_id: Option<Id>) -> sqlx::Result<Self> {
        let query_str = format!("
            INSERT INTO {link} ({left_id_str}, {right_id_str}, link_id )
            VALUES ($1, $2, $3)
            RETURNING *
            ",
            link = <Self as Model>::table(),
            left_id_str = <Self::Left as Model>::id_str(),
            right_id_str = <Self::Right as Model>::id_str(),);
        let res = sqlx::query_as::<Postgres, Self>(&query_str)
            .bind(left_id)
            .bind(right_id)
            .bind(link_id)
            .fetch_one(db).await?;
        Ok(res)

    }
    async fn delete_links_between(db: &PgPool, left_id: Id, right_id: Id) -> sqlx::Result<Vec<Self>> {
        let query_str = format!("
            DELETE FROM {link}
            WHERE {left_id_str} = $1
              AND {right_id_str} = $2
            RETURNING * ",
            link = <Self as Model>::table(),
            left_id_str = <Self::Left as Model>::id_str(),
            right_id_str = <Self::Right as Model>::id_str(),);
        let res = sqlx::query_as::<Postgres, Self>(&query_str)
            .bind(left_id)
            .bind(right_id)
            .fetch_all(db).await?;
        Ok(res)

    }
    async fn links_between(db: &PgPool, left_id: Id, right_id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("
                SELECT * FROM {link}
                WHERE {left_id_str} = $1
                 AND  {right_id_str} = $2",
            link = Self::table(),
            left_id_str = Self::Left::id_str(),
            right_id_str = Self::Right::id_str()))
            .bind(left_id)
            .bind(right_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    async fn service_links_between(db: Data<Db>, id: Path<(Id, Id)>) -> actix_web::Result<HttpResponse> {
        let (left_id, right_id) = id.into_inner();
        match Self::links_between(&db.pool, left_id, right_id).await {
            Ok(links) => Ok(respond::ok(links)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_insert_link_between(db: Data<Db>, id: Path<(Id, Id)>) -> actix_web::Result<HttpResponse> {
        let (left_id, right_id) = id.into_inner();
        match Self::insert_link_between(&db.pool, left_id, right_id, None).await {
            Ok(link) => Ok(respond::ok(link)),
            Err(e) => Ok(respond::err(e)),
        }

    }
    async fn service_delete_links_between(db: Data<Db>, id: Path<(Id, Id)>) -> actix_web::Result<HttpResponse> {
        let (left_id, right_id) = id.into_inner();
        match Self::delete_links_between(&db.pool, left_id, right_id).await {
            Ok(link) => Ok(respond::ok(link)),
            Err(e) => Ok(respond::err(e)),
        }

    }

}
