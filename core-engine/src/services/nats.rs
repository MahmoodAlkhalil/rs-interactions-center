use axum::http::StatusCode;
use core_engine_dto::{
    errors::IcError,
    nats::{KeyPair, NatsUsersConfig, Permissions, User},
};
use std::process::Command;
use std::vec;
use std::{env, fs};
use tracing::error;

pub fn generate_nkeys() -> Result<KeyPair, IcError> {
    let output = Command::new(env::var("NATS_NK_BIN_PATH").unwrap())
        .args(&["-gen", "user", "-pubout"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(
            "failed to generate nats user seed and pub key, std:io errpr [{}]",
            stderr
        );
        return Err(IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "faield to generate nats user, check logs".to_string(),
        });
    }

    let output_str = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = output_str.lines().collect();

    if lines.len() != 2 {
        return Err(IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "faield to generate nats user, check logs".to_string(),
        });
    }
    let key_pair = KeyPair {
        seed: lines[0].to_string(),
        pub_key: lines[1].to_string(),
    };

    Ok(key_pair)
}

pub fn append_to_users_conf(user_id: &str, pub_key: &str) -> Result<(), IcError> {
    let users_config_file_path = env::var("NATS_USERS_CONFIG_PATH");
    let users_config_file_path = match users_config_file_path {
        Ok(data) => data,
        Err(_) => {
            error!("no value for NATS_USERS_CONFIG_PATH env variable");
            return Err(IcError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "server error, check logs".to_string(),
            });
        }
    };

    let content = fs::read_to_string(users_config_file_path.as_str());
    let content = match content {
        Ok(data) => data,
        Err(_) => {
            error!(
                "failed to read NATS users config file from path [{}]",
                users_config_file_path.as_str()
            );
            return Err(IcError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "server error, check logs".to_string(),
            });
        }
    };

    let users_config: Result<NatsUsersConfig, hcl::Error> = hcl::from_str(content.as_str());
    let mut users_config = match users_config {
        Ok(data) => data,
        Err(_) => {
            error!(
                "failed parsing file [{}] into HCL format",
                users_config_file_path.as_str()
            );
            return Err(IcError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "server error, check logs".to_string(),
            });
        }
    };
    users_config.authorization.users.push(User {
        nkey: format!("{}", pub_key),
        permissions: Some(Permissions {
            sub: Some(vec![format!("agent.{}", user_id)]),
            r#pub: Some(vec![format!("agent.{}", user_id)]),
        }),
    });
    let users_config = hcl::to_string(&users_config);
    let users_config = match users_config {
        Ok(data) => data,
        Err(error) => {
            error!(
                "failed to serialize users_config struct into a string. {}",
                error
            );
            return Err(IcError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "server error, check logs".to_string(),
            });
        }
    };
    let write_result = fs::write(users_config_file_path.as_str(), users_config);
    match write_result {
        Ok(_) => return Ok(()),
        Err(_) => {
            error!("failed to write new users config file to disk");
            return Err(IcError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "server error, check logs".to_string(),
            });
        }
    };
}
