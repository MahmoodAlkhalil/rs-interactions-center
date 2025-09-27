use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSkill {
    pub name: String,
    pub parent_skill_id:Option<Uuid>
}
