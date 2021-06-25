pub mod jwt;
pub mod session;
// pub mod guard;

pub struct AuthRequest {

}
pub struct UserInfo {
    user_id: Option<i32>,
    username: String,
    email: String,
}
