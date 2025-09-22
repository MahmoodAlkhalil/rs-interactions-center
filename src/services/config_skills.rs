use crate::dtos::shared::ServiceDto;
use crate::dtos::skills::requests::CreateSkill;
use crate::shared::IcError;
use crate::{db, dtos};
use axum::Json;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn get_all<T>(db: &T) -> Result<Json<Vec<dtos::skills::responses::Skill>>, IcError>
where
    T: ConnectionTrait + TransactionTrait,
{
    let skills = db::repo::config_skills::get_all(db).await?;
    let mut response = vec![];
    for skill in skills {
        response.push(dtos::skills::responses::Skill {
            id: skill.id,
            name: skill.name,
            created_at: skill.created_at.to_utc(),
        })
    }
    Ok(Json(response))
}

pub async fn create<B>(
    request: ServiceDto<'_, CreateSkill, B>,
) -> Result<Json<dtos::skills::responses::Skill>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let new_queue = db::repo::config_skills::create(request).await?;
    let response = dtos::skills::responses::Skill {
        id: new_queue.id,
        name: new_queue.name,
        created_at: new_queue.created_at.to_utc(),
    };
    Ok(Json(response))
}
