use sea_orm_migration::{prelude::*, schema::*};
use tracing::info;
use uuid::uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("channels")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name"))
                    .col(text("description"))
                    .col(timestamp_with_time_zone("created_at").default("now()"))
                    .col(boolean("mark_for_delete").default(false))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("groups")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name"))
                    .col(text("description"))
                    .col(timestamp_with_time_zone("created_at"))
                    .col(boolean("mark_for_delete").default(false))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("interaction_states")
                    .if_not_exists()
                    .col(integer("id").unique_key())
                    .col(text_uniq("name"))
                    .col(text("description"))
                    .col(timestamp_with_time_zone("created_at").not_null())
                    .col(boolean("mark_for_delete").default(false).not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("interactions")
                    .col(pk_uuid("id"))
                    .col(integer("state").not_null())
                    .col(
                        timestamp_with_time_zone("created_at")
                            .not_null()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("interactions_channels_assignment")
                    .if_not_exists()
                    .col(uuid("interaction_id").not_null())
                    .col(uuid("channel_id").not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_interactions_channels_assignment_interaction_id")
                            .from("interactions_channels_assignment", "interaction_id")
                            .to("interactions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_interactions_channels_assignment_channel_id")
                            .from("interactions_channels_assignment", "channel_id")
                            .to("channels", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("interactions_skills_assignment")
                    .if_not_exists()
                    .col(uuid("interaction_id").not_null())
                    .col(uuid("skill_id").not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_interactions_skills_assignment_interaction_id")
                            .from("interactions_skills_assignment", "interaction_id")
                            .to("interactions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_interactions_skills_assignment_skill_id")
                            .from("interactions_skills_assignment", "skill_id")
                            .to("skills", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("queues")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name").not_null())
                    .col(
                        timestamp_with_time_zone("created_at")
                            .not_null()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("queues_channels_assignment")
                    .if_not_exists()
                    .col(uuid("queue_id").not_null())
                    .col(uuid("channel_id").not_null())
                    .col(
                        timestamp_with_time_zone("created_at")
                            .not_null()
                            .default("now()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queues_channels_assignment_queue_id")
                            .from("queues_channels_assignment", "queue_id")
                            .to("queues", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queues_channels_assignment_channel_id")
                            .from("queues_channels_assignment", "channel_id")
                            .to("channels", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("queues_groups_assignment")
                    .if_not_exists()
                    .col(uuid("queue_id").not_null())
                    .col(uuid("group_id").not_null())
                    .col(
                        timestamp_with_time_zone("created_at")
                            .not_null()
                            .default("now()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queues_groups_assignment_queue_id")
                            .from("queues_groups_assignment", "queue_id")
                            .to("queues", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("queues_groups_assignment", "group_id")
                            .to("groups", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("runtime_interactions_queues")
                    .to_owned()
                    .if_not_exists()
                    .col(uuid("queue_id"))
                    .col(uuid("interaction_id"))
                    .col(integer("priority").default(50))
                    .col(timestamp_with_time_zone("queued_at").default("now()"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("runtime_users")
                    .to_owned()
                    .if_not_exists()
                    .col(uuid("id"))
                    .col(integer("state"))
                    .col(timestamp_with_time_zone("state_updated_at").default("now()"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("runtime_users_queues")
                    .to_owned()
                    .if_not_exists()
                    .col(uuid("queue_id"))
                    .col(integer("user_id"))
                    .col(timestamp_with_time_zone("queued_at").default("now()"))
                    .col(boolean("on_queue"))
                    .to_owned(),
            )
            .await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
