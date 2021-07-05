# devisa_api
## Part of the Devisa Core API

---
### News
- No news!


---
### About
- Responsible for operations falling under the broad umbrella of real-time network communications

#### Route automation structure
- ^[07/04/21]^ ^[17:39]^ **Route and handlers are automatically implemented** for models by implementing the `crate::models::Model` trait. Once implemented, the Model trait produces CRUD handler functions for the struct, and produces a `Model::scope()` function which acts as a `actix_web::Scope` service, automatically generating and encapsulating appropriate services for each of the auto-generated CRUD handlers.
    - Structs which implement the `Model` trait may also add additional custom routes (and corresponding custom handlers) by implementing the `Model::routes()` trait method, which is automatically configured for the exported `Model::scope()` trait method which is then used by other, more base-level route encapsulation methods (i.e. root routes handler).
- ^[07/04/21]^ ^[17:43]^ **Models linked to another model by a join table**, may also have automated route generation by implementing the `LinkedTo<L>` trait, where `L` is the struct which the model is linked to. `LinkedTo<L>` exports a `scope()` method as well, which contains an `actix_web::Scope` service encapsulating routes and their corresponding handlers for CRUD methods on the linked struct with regards to the struct implementing the trait.
- ^[07/04/21]^ ^[17:47]^ **Structs corresponding to a join table** implement both `Model` and `Linked`, where `Linked` has associated types `Linked::Left` and `Linked::Right` in a non-arbitrary ordering. For example, a join table struct `GroupUser` links `Group` implementing `LinkedTo<User>` and `User` implementing `LinkedTo<Group>`. `GroupUser` then implements `Linked`.
    - `Linked` exports a `scope()` method, providing a service which handles requests to paths of type `/{left path}/:left_id/{right path}/:right_id`, and provides its service **only** at this path. The reverse path order, `/{right path}/:right_id/{left path}/:left_id` is not implemented (would correspond to a "UserGroup" struct).
    - This is important, because while both the left and right associated models both implement `LinkedTo<Other>`, and therefore generate and export a service handling requests at `/{self path}/:self_id/{other path}`, when it comes to handling `/{self path}/:self_id/{other path}/:other_id`, only one arrangement will produce responses, which is reflected in the Left/Right designation in the `Linked` implementation for the join table.
    - The purpose of this is to reduce redundancy. While it is essential to be able to get linked structs from either end of a join table in either direction, getting the _links themselves_ (which is what is handled and returned by the `Linked` trait implementation) has no direction component, and so implementation in both directions is redundant.
- ^[07/04/21]^ ^[17:56]^ The full route service mapping provided by each trait, after proper implementation can be seen as follows:
```
<Group as Model>              :   "/group"
<GroupUser as Model>          :   "/group/user"
<Group as LinkedTo<User>>     :   "/group/{group_id}/user"
<GroupUser as Linked>         :   "/group/{group_id}/user/{user_id}"
```


---
### Notes
- [ ] ^[07/03/21]^ ^[07:41]^ **TODO**: Automate API routes so that models and corresponding operations, esp. those standardized across models, can be automated. I.e., extract `User` model designation from `"/user"`, then match `POST` requests to some `create()` method which requires the user model in Json spec to insert into Db, to reduce massive boilerplate.


---
### Links
- [dv_api Root](.) **(You are here!)**:
    - [api-srv](api-srv/README.md)
        - [api-actix](api-srv/api-actix/README.md)
        - [api-tide](api-srv/api-tide/README.md)
        - [api-warp](api-srv/api-warp/README.md)
    - [api-redis](api-redis/README.md)

---
### To run
- ^[06/26/21]^ ^[22:28]^ To run from api server:
```
sudo podman run --name di-redis -d docker.io/redis:latest
sudo podman run --name di-api
    \ -p "80:80"
    \ --env-file .env
    \ quay.io/devisa/devisa-api:latest
```


---
### Todo
