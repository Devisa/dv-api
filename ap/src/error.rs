use serde::{Serialize, Deserialize, ser::{Serializer, SerializeStruct}};
use std::{fmt, error};
use ap_com::DiLibError;
use derive_more::{Error, From, Into, Display};
use actix_web::{
    http::StatusCode,
    error::{ResponseError, JsonPayloadError, QueryPayloadError},
    HttpResponse, HttpRequest,
};
use serde_json::Value;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Token error")]
    TokenError { token: String },
    #[display(fmt = "Missing param")]
    MissingParam { param: String },
    #[display(fmt = "Path not found")]
    PathNotFound { path: String },
    #[display(fmt = "Object not found")]
    ObjectNotFound { object: String },
    #[display(fmt = "Auth error")]
    AuthError,
    #[display(fmt = "Internal server error")]
    InternalError(String),
    #[display(fmt = "Parse error")]
    ParseError(ParseError),
    #[display(fmt = "Db error")]
    DbError(sqlx::Error),
    #[display(fmt = "JsonRPC error")]
    JsonRpcError(JsonRpcError),
}

#[derive(Debug, Display, Error)]
pub enum ParseError {
    Json(serde_json::Error),
    Uuid(uuid::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: serde_json::Value,
}

impl JsonRpcError {
    pub fn new(code: i32, message: &str) -> Self {
        Self {
            code, message: String::from(message), data: Value::Null,
        }
    }
    pub fn std(code: i32) -> Self {
        match code {
            -32700 => Self::new(-32700, "Parse Error"),
            -32600 => Self::new(-32600, "Invalid Request"),
            -32601 => Self::new(-32601, "Method Not Found"),
            -32602 => Self::new(-32602, "Invalid Params"),
            -32603 => Self::new(-32603, "Internal Error"),
            _ => panic!("Undefined error code"),
        }
    }
    pub fn dump(&self) -> String {
        serde_json::to_string(self).expect("Error")
    }
}

impl error::Error for JsonRpcError {}
impl fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.code, self.message, self.data)
    }
}

impl ResponseError for JsonRpcError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self.code {
            -32700 => StatusCode::INTERNAL_SERVER_ERROR,
            -32600 => StatusCode::BAD_REQUEST,
            -32601 => StatusCode::METHOD_NOT_ALLOWED,
            -32602 => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::Ok()
            .finish()
    }
}


pub type ApiResult<T> = Result<T, ApiError>;

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            Self::PathNotFound { path } => StatusCode::BAD_GATEWAY,
            Self::TokenError { token }  => StatusCode::UNAUTHORIZED,
            Self::MissingParam { param } => StatusCode::BAD_REQUEST,
            Self::ObjectNotFound { object } => StatusCode::NOT_FOUND,
            Self::DbError(e) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::Ok()
            .finish()
    }
}

#[derive(Debug, Display, Error, From)]
pub enum TokenError {
    MissingToken,
    InvalidToken,
    Internal,
}
impl ResponseError for TokenError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            TokenError::MissingToken => StatusCode::NOT_FOUND,
            TokenError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            TokenError::InvalidToken => StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            TokenError::MissingToken => HttpResponse::NotFound().finish(),
            TokenError::Internal => HttpResponse::InternalServerError().finish(),
            TokenError::InvalidToken => HttpResponse::NotAcceptable().finish(),
        }
    }
}


impl std::error::Error for ApiError {}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> ApiError {
        use sqlx::Error;
        match err {
            Error::Io(e) => Self::InternalError(e.to_string()),
            Error::Tls(e) => Self::InternalError(e.to_string()),
            Error::Decode(e) => Self::InternalError(e.to_string()),
            Error::Migrate(e) => Self::InternalError("".to_string()),
            Error::Database(e) => Self::InternalError(e.to_string()),
            Error::Protocol(e) => Self::InternalError(e),
            Error::RowNotFound => Self::InternalError("".to_string()),
            Error::TypeNotFound { type_name } => Self::InternalError(type_name),
            _ => Self::InternalError("".to_string()),
        }
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(err: actix_web::Error) -> ApiError {
        match err {
            _ => Self::InternalError(err.to_string()),
        }
    }
}

impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> ApiError {
        ApiError::ParseError(ParseError::Uuid(err))
    }
}
impl From<serde_json::error::Error> for ApiError {
    fn from(err: serde_json::error::Error) -> ApiError {
        ApiError::ParseError(ParseError::Json(err))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::InternalError(err.to_string())
    }
}


pub async fn error_resp(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    return Ok(HttpResponse::BadRequest()
            .insert_header(("Content-Type", "application/json"))
            .body(format!(
                    "Internal error: Inserted creds, but one failure:\n
                     User insert: {}\n
                     Acct insert: {}\n
                     Profile insert: {}\n",
                     true, true, true,
             )))

}
