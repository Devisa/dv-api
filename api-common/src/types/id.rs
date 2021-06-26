use derive_more::{From, Display, FromStr, Error};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Clone,  Display)]
#[sqlx(transparent, type_name = "id")]
pub struct Id(String);

impl Id {

    pub fn gen() -> Self {
        Self(Uuid::new_v4().to_string())
    }

}

impl From<String> for Id {
    fn from(string: String) -> Self {
        Self(Uuid::parse_str(&string)
            .map_err(|e| {
                tracing::info!("Eror decoding UUID: {}", e);
            })
            .unwrap_or(Uuid::nil())
            .to_string()
        )
    }
}
