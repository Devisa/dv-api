use derive_more::{From, Display, AsMut, AsRef, FromStr, Error};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Clone, Display, AsRef, AsMut)]
#[sqlx(transparent, type_name = "id")]
pub struct Id(String);

impl Id {

    pub fn gen() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn nil() -> Self {
        Self(Uuid::nil().to_string())
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

impl From<Uuid> for Id {
    fn from(guid: Uuid) -> Self {
        Self(guid.to_string())
    }
}
