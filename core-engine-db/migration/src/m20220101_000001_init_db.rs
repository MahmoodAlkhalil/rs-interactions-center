use core_engine_const::interaction_states::InteractionStates;
use core_engine_const::user_states::UserStates;
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{EntityTrait, Set, TransactionTrait};
use sea_orm::{ExecResult, Statement};
use sea_orm_migration::{prelude::*, schema::*};
use strum::{EnumProperty, VariantArray};
use tracing::info;
use uuid::{Uuid, uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("creating interaction_states");
        manager
            .create_table(
                Table::create()
                    .table("interaction_states")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name"))
                    .col(text_null("description"))
                    .col(boolean("system_state").default(false))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        println!("creating user_states");
        manager
            .create_table(
                Table::create()
                    .table("user_states")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name"))
                    .col(text_null("description"))
                    .col(uuid_null("parent_id"))
                    .col(boolean("mark_for_delete").default(false))
                    .col(boolean("system_state").default(false))
                    .col(boolean("user_control_allowed").default(true))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from("user_states", "parent_id")
                            .to("user_states", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("creating channels");
        manager
            .create_table(
                Table::create()
                    .table("channels")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name"))
                    .col(text_null("description"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(boolean("mark_for_delete").default(false))
                    .col(boolean("online").default(false))
                    .index(Index::create().col("id").unique())
                    .to_owned(),
            )
            .await?;

        println!("creating groups");
        manager
            .create_table(
                Table::create()
                    .table("groups")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name"))
                    .col(text_null("description"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(boolean("mark_for_delete").default(false))
                    .index(Index::create().col("id").unique())
                    .to_owned(),
            )
            .await?;
        println!("creating interactions");
        manager
            .create_table(
                Table::create()
                    .table("interactions")
                    .col(pk_uuid("id"))
                    .col(uuid("state"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .index(Index::create().col("id").unique())
                    .to_owned(),
            )
            .await?;

        println!("creating skills");
        manager
            .create_table(
                Table::create()
                    .table("skills")
                    .to_owned()
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text("name"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(uuid_null("parent_id"))
                    .col(boolean("mark_for_delete").default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .from("skills", "parent_id")
                            .to("skills", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(Index::create().col("id").unique())
                    .to_owned(),
            )
            .await?;

        println!("creating users");
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .to_owned()
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text("username"))
                    .col(text("name").unique_key())
                    .col(uuid("state"))
                    .col(text("nkey_seed").unique_key())
                    .col(text("nkey_pub").unique_key())
                    .col(boolean("service_account").default(false))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(boolean("mark_for_delete").default(false))
                    .index(Index::create().col("id").unique())
                    .to_owned(),
            )
            .await?;

        println!("creating queues");
        manager
            .create_table(
                Table::create()
                    .table("queues")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(text_uniq("name").not_null())
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(boolean("mark_for_delete").default(false))
                    .index(Index::create().col("id").unique())
                    .to_owned(),
            )
            .await?;

        println!("creating interactions_events");
        manager
            .create_table(
                Table::create()
                    .table("interactions_events")
                    .col(big_pk_auto("internal_id"))
                    .col(uuid("id"))
                    .col(integer("type"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(uuid_null("old_state"))
                    .col(uuid_null("new_state"))
                    .col(uuid_null("queue_id"))
                    .col(uuid_null("user_id"))
                    .col(uuid_null("channel_id"))
                    .to_owned(),
            )
            .await?;

        println!("creating interactions_channels_assignment");
        manager
            .create_table(
                Table::create()
                    .table("interactions_channels_assignment")
                    .if_not_exists()
                    .col(uuid("interaction_id"))
                    .col(uuid("channel_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("interaction_id").col("channel_id"))
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

        println!("creating interactions_skills_assignment");
        manager
            .create_table(
                Table::create()
                    .table("interactions_skills_assignment")
                    .if_not_exists()
                    .col(uuid("interaction_id"))
                    .col(uuid("skill_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("interaction_id").col("skill_id"))
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

        println!("creating queues_channels_assignment");
        manager
            .create_table(
                Table::create()
                    .table("queues_channels_assignment")
                    .if_not_exists()
                    .col(uuid("queue_id"))
                    .col(uuid("channel_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("queue_id").col("channel_id"))
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

        println!("creating queues_groups_assignment");
        manager
            .create_table(
                Table::create()
                    .table("queues_groups_assignment")
                    .if_not_exists()
                    .col(uuid("queue_id"))
                    .col(uuid("group_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("queue_id").col("group_id"))
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
                    .col(integer("priority"))
                    .col(timestamp_with_time_zone("queued_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("queue_id").col("interaction_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("runtime_interactions_queues", "queue_id")
                            .to("queues", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("runtime_interactions_queues", "interaction_id")
                            .to("interactions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("runtime_users")
                    .to_owned()
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("state"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(
                        timestamp_with_time_zone("state_updated_at")
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("runtime_users", "id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
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
                    .col(uuid("user_id"))
                    .col(timestamp_with_time_zone("queued_at").default(Expr::current_timestamp()))
                    .col(boolean("on_queue").default(false))
                    .primary_key(Index::create().col("queue_id").col("user_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("runtime_users_queues", "queue_id")
                            .to("queues", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("runtime_users_queues", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("skills_groups_assignment")
                    .to_owned()
                    .if_not_exists()
                    .col(uuid("skill_id"))
                    .col(uuid("group_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("skill_id").col("group_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("skills_groups_assignment", "skill_id")
                            .to("skills", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("skills_groups_assignment", "group_id")
                            .to("groups", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("users_groups_assignment")
                    .to_owned()
                    .if_not_exists()
                    .col(uuid("user_id"))
                    .col(uuid("group_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("user_id").col("group_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("users_groups_assignment", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("users_groups_assignment", "group_id")
                            .to("groups", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("users_queues_assignment")
                    .to_owned()
                    .if_not_exists()
                    .col(uuid("user_id"))
                    .col(uuid("queue_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("user_id").col("queue_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("users_queues_assignment", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("users_queues_assignment", "queue_id")
                            .to("queues", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("users_skills_assignment")
                    .to_owned()
                    .if_not_exists()
                    .col(uuid("user_id"))
                    .col(uuid("skill_id"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("user_id").col("skill_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("users_skills_assignment", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("users_skills_assignment", "skill_id")
                            .to("skills", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("inserting user states");
        let mut offline_uuid: Option<Uuid> = None;
        for state in UserStates::VARIANTS.into_iter() {
            let state_id = Uuid::now_v7();
            let state_name = state.to_string();
            if state.to_string() == "Offline" {
                offline_uuid = Some(state_id);
            }
            manager
                .get_connection()
                .execute_unprepared(
                    format!(
                        "INSERT INTO user_states (id, name, system_state) VALUES ('{}','{}',true)",
                        state_id, state_name
                    )
                    .as_str(),
                )
                .await?;
        }

        manager
            .get_connection()
            .execute_unprepared(CREATE_USER_STATES_MATERIALIZED_VIEW)
            .await?;

        println!("inserting interaction states");
        for state in InteractionStates::VARIANTS.into_iter() {
            let state_id = Uuid::now_v7();
            let state_name = state.to_string();
            manager.get_connection().execute_unprepared(
                format!(
                    "INSERT INTO interaction_states (id, name, system_state) VALUES ('{}','{}',true)",
                    state_id, state_name
                )
                .as_str(),
            )
            .await?;
        }

        println!("inserting channels");
        manager.get_connection().execute_unprepared("INSERT INTO channels(id, name) VALUES ('0199d191-78f6-7153-8eb9-b4b926a95996', 'Voice')").await?;
        manager.get_connection().execute_unprepared("INSERT INTO queues(id, name) VALUES ('0199d191-2976-7313-9f51-85c0d31420c9','Voice Sample Queue 1')")
            .await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO queues(id, name) VALUES ('0199d1b1-ada5-7179-aff4-e72f75148018','Voice Sample Queue 2')")
            .await?;
        manager.get_connection().execute_unprepared("INSERT INTO queues_channels_assignment(queue_id, channel_id) VALUES ('0199d191-2976-7313-9f51-85c0d31420c9','0199d191-78f6-7153-8eb9-b4b926a95996')")
            .await?;
        manager.get_connection().execute_unprepared("INSERT INTO queues_channels_assignment(queue_id, channel_id) VALUES ('0199d1b1-ada5-7179-aff4-e72f75148018','0199d191-78f6-7153-8eb9-b4b926a95996')")
        .await?;

        println!("inserting users");
        if offline_uuid.is_some() {
            manager.get_connection().execute_unprepared(
            format!(r#"INSERT INTO users(id, username, name, nkey_seed, nkey_pub,service_account, state) 
            VALUES ('{}','core_engine_svc','core engine service account','SUAINSF3V32D5P6TLUQCFCFONRPRM5EDVISYZVTTU75WUHWL4FWBL7YDNE', 'UDMF5NAGRQGFFCZOG6RDLJUTKIOBIPAGK3MCSJVJALXTXLINUVUCPX2N', true, '{}')"#,Uuid::now_v7(), offline_uuid.unwrap()).as_str())
            .await?;

            manager.get_connection().execute_unprepared(
                format!(  r#"INSERT INTO users(id, username,name, nkey_seed, nkey_pub, state) 
            VALUES ('{}', 'agent1', 'Sample Agent 1','SUAHMSVN6476ELYZUOSKWK3LSTJ72MCXGDZZVQQXKOO7574F4OMPVEY53E', 'UA6ANCGHBXK2N4QOVYUGULXQXFTOPF2YALZTHTFFTF75APFRRS7VQE5P', '{}')"#,Uuid::now_v7(),offline_uuid.unwrap()).as_str())
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP MATERIALIZED VIEW user_states_tree_mv;")
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("interactions_events")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("users_skills_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("users_queues_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("users_groups_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("skills_groups_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("runtime_users_queues")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("runtime_users").if_exists().to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("runtime_interactions_queues")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("queues_groups_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("queues_channels_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("queues").if_exists().to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("interactions_skills_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("interactions_channels_assignment")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("user_states").if_exists().to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table("interaction_states")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("users").if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("skills").if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("interactions").if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("groups").if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("channels").if_exists().to_owned())
            .await?;

        Ok(())
    }
}
const CREATE_USER_STATES_MATERIALIZED_VIEW: &str = r#"
CREATE MATERIALIZED VIEW user_states_tree_mv AS
WITH RECURSIVE state_tree AS (
    SELECT
        us.id,
        us.name,
        us.description,
        us.parent_id,
        us.mark_for_delete,
        us.system_state,
        us.user_control_allowed,
        us.created_at,
        0 AS level,
        ARRAY[us.id] AS path,
        us.name::text AS full_path
    FROM
        user_states us
    WHERE
        us.parent_id IS NULL
        AND us.mark_for_delete = false

    UNION ALL

    SELECT
        us.id,
        us.name,
        us.description,
        us.parent_id,
        us.mark_for_delete,
        us.system_state,
        us.user_control_allowed,
        us.created_at,
        st.level + 1 AS level,
        st.path || us.id AS path,
        st.full_path || ' -> ' || us.name AS full_path
    FROM
        user_states us
    INNER JOIN
        state_tree st ON us.parent_id = st.id
    WHERE
        us.mark_for_delete = false
)
SELECT
    st.id,
    st.name,
    st.description,
    st.parent_id,
    st.mark_for_delete,
    st.system_state,
    st.user_control_allowed,
    st.created_at,
    st.level,
    st.path,
    st.full_path
FROM
    state_tree st
ORDER BY
    st.path;
CREATE INDEX idx_user_states_tree_mv_id ON user_states_tree_mv (id);
CREATE INDEX idx_user_states_tree_mv_parent_id ON user_states_tree_mv (parent_id);
CREATE INDEX idx_user_states_tree_mv_path ON user_states_tree_mv USING gin (path);
CREATE INDEX idx_user_states_tree_mv_level ON user_states_tree_mv (level);
REFRESH MATERIALIZED VIEW user_states_tree_mv;
"#;
