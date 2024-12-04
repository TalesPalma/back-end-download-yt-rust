use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Musica {
    pub id: String,
    pub title: String,
    pub url: String,
}
