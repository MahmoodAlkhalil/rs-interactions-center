use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateChannel {
    pub name: String,
}
