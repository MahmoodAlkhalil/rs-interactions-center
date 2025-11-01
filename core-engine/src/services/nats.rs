use std::{collections::HashMap, env};

use async_nats::Client;
use axum_jwks::Claims;
use core_engine_const::defaults::{
    NATS_CORE_ENGINE_JWT_CONFIG_KEY, NATS_CORE_ENGINE_SEED_CONFIG_KEY, NATS_DEFAULT_JWT_CONFIG_KEY,
    NATS_DEFAULT_SEED_CONFIG_KEY, NATS_OPERATOR_JWT_CONFIG_KEY, NATS_OPERATOR_SEED_CONFIG_KEY,
    NATS_SYS_JWT_CONFIG_KEY, NATS_SYS_SEED_CONFIG_KEY,
};

use core_engine_dto::errors::ApiError;
use sea_orm::{
    ActiveModelBehavior, ActiveValue::Set, ColIdx, ConnectionTrait, DatabaseConnection,
    EntityTrait, QueryFilter, TransactionTrait,
};
use tracing::{info, warn};
use tracing_subscriber::fmt::format;
use uuid::Uuid;

use core_engine_db::entities::system_config::ActiveModel as SystemConfigAM;
use core_engine_db::entities::system_config::Column as SystemConfigC;
use core_engine_db::entities::system_config::Entity as SystemConfigE;
use nats_io_jwt::{
    Account, ConnectionType, KeyPair, Operator, OperatorLimits, Permission, SigningKeys,
    SystemAccount, Token, User,
};
use sea_orm::ColumnTrait;

use crate::utils::nats::NatsConfig;

pub async fn create_client(db: &DatabaseConnection) -> Result<Client, ApiError> {
    let jwt = SystemConfigE::find()
        .filter(SystemConfigC::Key.eq(NATS_CORE_ENGINE_JWT_CONFIG_KEY))
        .one(db)
        .await?
        .ok_or(ApiError::internal_error(
            "core-engine NATS jwt is not generated",
        ))?
        .val;
    let seed = SystemConfigE::find()
        .filter(SystemConfigC::Key.eq(NATS_CORE_ENGINE_SEED_CONFIG_KEY))
        .one(db)
        .await?
        .ok_or(ApiError::internal_error(
            "core-engine NATS seed is not generated",
        ))?
        .val;

    let nats_url = env::var("NATS_URL").unwrap();
    let key_pair = std::sync::Arc::new(nkeys::KeyPair::from_seed(&seed).unwrap());
    let nc = async_nats::ConnectOptions::with_jwt(jwt.to_owned(), move |nonce| {
        let key_pair = key_pair.clone();
        async move { key_pair.sign(&nonce).map_err(async_nats::AuthError::new) }
    })
    .connect(nats_url)
    .await?;
    Ok(nc)
}

pub async fn init_config(db: &DatabaseConnection) -> Result<(), ApiError> {
    let tx = db.begin().await?;
    let opeartor_jwt = SystemConfigE::find()
        .filter(SystemConfigC::Key.eq(NATS_OPERATOR_JWT_CONFIG_KEY))
        .one(&tx)
        .await?;

    if opeartor_jwt.is_some() {
        warn!("NATS Operator is already generated");
        return Ok(());
    }

    let op_key = KeyPair::new_operator();
    let sys_key = KeyPair::new_account();
    let sys_s_key = KeyPair::new_account();
    let default_key = KeyPair::new_account();
    let core_engine_key = KeyPair::new_user();

    let op_pub = op_key.public_key();
    let sys_pub = sys_key.public_key();
    let sys_s_pub = sys_s_key.public_key();
    let default_pub = default_key.public_key();
    let core_engine_pub = core_engine_key.public_key();

    let sys_claims: Account = Account::builder()
        .signing_keys(SigningKeys::from(&sys_key))
        .limits(Some(OperatorLimits {
            ..Default::default()
        }))
        .try_into()?;
    let sys_token = Token::new(sys_pub.clone())
        .claims(sys_claims)
        .name("SYS")
        .sign(&op_key);

    let default_claims: Account = Account::builder()
        .signing_keys(SigningKeys::from(&default_key))
        .limits(Some(OperatorLimits {
            ..Default::default()
        }))
        .try_into()?;
    let default_token = Token::new(default_pub.clone())
        .claims(default_claims)
        .name("DEFAULT")
        .sign(&op_key);

    let op_claims: Operator = Operator::builder()
        .system_account(SystemAccount::from(sys_pub.clone()))
        .try_into()?;

    let op_token = Token::new(op_pub.clone())
        .claims(op_claims)
        .name("rs-interactions-center")
        .sign(&op_key);

    let core_engine_claims: User = User::builder()
        .pub_(Permission::from(">"))
        .sub(Permission::from(">"))
        .allowed_connection_types(vec![ConnectionType::Standard])
        .payload(1024 * 1024 * 20)
        .bearer_token(true)
        .try_into()?;

    let core_engine_token = Token::new(core_engine_pub.clone())
        .claims(core_engine_claims)
        .name("core-engine")
        .sign(&default_key);

    let mut seed_row = SystemConfigAM::new();
    seed_row.key = Set(NATS_OPERATOR_SEED_CONFIG_KEY.to_owned());
    seed_row.val = Set(op_key.seed().unwrap());

    let mut token_row = SystemConfigAM::new();
    token_row.key = Set(NATS_OPERATOR_JWT_CONFIG_KEY.to_owned());
    token_row.val = Set(op_token.clone());

    SystemConfigE::insert_many([seed_row, token_row])
        .exec(&tx)
        .await?;

    let mut seed_row = SystemConfigAM::new();
    seed_row.key = Set(NATS_SYS_SEED_CONFIG_KEY.to_owned());
    seed_row.val = Set(sys_key.seed().unwrap());

    let mut token_row = SystemConfigAM::new();
    token_row.key = Set(NATS_SYS_JWT_CONFIG_KEY.to_owned());
    token_row.val = Set(default_token.clone());

    SystemConfigE::insert_many([seed_row, token_row])
        .exec(&tx)
        .await?;

    let mut seed_row = SystemConfigAM::new();
    seed_row.key = Set(NATS_DEFAULT_SEED_CONFIG_KEY.to_owned());
    seed_row.val = Set(default_key.seed()?);

    let mut token_row = SystemConfigAM::new();
    token_row.key = Set(NATS_DEFAULT_JWT_CONFIG_KEY.to_owned());
    token_row.val = Set(default_token.clone());

    SystemConfigE::insert_many([seed_row, token_row])
        .exec(&tx)
        .await?;

    let mut seed_row = SystemConfigAM::new();
    seed_row.key = Set(NATS_CORE_ENGINE_SEED_CONFIG_KEY.to_owned());
    seed_row.val = Set(core_engine_key.seed()?);

    let mut token_row = SystemConfigAM::new();
    token_row.key = Set(NATS_CORE_ENGINE_JWT_CONFIG_KEY.to_owned());
    token_row.val = Set(core_engine_token);

    SystemConfigE::insert_many([seed_row, token_row])
        .exec(&tx)
        .await?;

    let mut resolver_preload: HashMap<String, String> = HashMap::with_capacity(1);
    resolver_preload.insert(sys_pub.clone(), sys_token.clone());
    resolver_preload.insert(default_pub.clone(), default_token.clone());
    let config = NatsConfig {
        operator: Some(op_token),
        system_account: Some(sys_pub.clone()),
        resolver_preload: Some(resolver_preload),
        resolver: Some("MEMORY".to_owned()),
        ..Default::default()
    };
    info!(
        "NATS config : \r\n {}",
        NatsConfig::to_hcl(&config).unwrap()
    );
    tx.commit().await?;
    Ok(())
}

pub async fn generate_user<T>(user_id: Uuid, db: &T) -> Result<(KeyPair, String), ApiError>
where
    T: ConnectionTrait + TransactionTrait,
{
    let tx = db.begin().await?;
    let default_account_signing_seed = SystemConfigE::find()
        .filter(SystemConfigC::Key.eq(NATS_DEFAULT_SEED_CONFIG_KEY))
        .one(&tx)
        .await?
        .ok_or(ApiError::internal_error(
            "default account NATS seed not defined",
        ))?
        .val;
    let signing_key = KeyPair::from_seed(&default_account_signing_seed)?;
    let user_keypair = KeyPair::new_user();
    let user: User = User::builder()
        .pub_(Permission::from(format!("user.{}.*", user_id)))
        .pub_(Permission::from("_INBOX.>>"))
        .pub_(Permission::from(format!("user.{}.*", user_id)))
        .allowed_connection_types(vec![ConnectionType::Standard, ConnectionType::Websocket])
        .payload(1024 * 1024 * 20)
        .bearer_token(true)
        .try_into()?;
    let user_token = Token::new(user_keypair.public_key())
        .name(user_id.to_string())
        .claims(user)
        .sign(&signing_key);
    Ok((user_keypair, user_token))
}

pub async fn generate_channel<T>(name: &str, db: &T) -> Result<(KeyPair, String), ApiError>
where
    T: ConnectionTrait + TransactionTrait,
{
    let tx = db.begin().await?;
    let default_account_signing_seed = SystemConfigE::find()
        .filter(SystemConfigC::Key.eq(NATS_DEFAULT_SEED_CONFIG_KEY))
        .one(&tx)
        .await?
        .ok_or(ApiError::internal_error(
            "default account NATS seed not defined",
        ))?
        .val;
    let signing_key = KeyPair::from_seed(&default_account_signing_seed)?;
    let user_keypair = KeyPair::new_user();
    let service: User = User::builder()
        .pub_(Permission::from(format!("channel.{}.events", name)))
        .pub_(Permission::from(format!("channel.{}.heartbeat", name)))
        .pub_(Permission::from("_INBOX.>>"))
        .allowed_connection_types(vec![ConnectionType::Standard, ConnectionType::Websocket])
        .payload(1024 * 1024 * 20)
        .bearer_token(true)
        .try_into()?;
    let service_token = Token::new(user_keypair.public_key())
        .name(name.to_string())
        .claims(service)
        .sign(&signing_key);
    Ok((user_keypair, service_token))
}
