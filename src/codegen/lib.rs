pub mod docs;
pub mod rust;
pub mod service;
pub mod sql;
use eyre::*;
use std::env;
use std::fs::create_dir_all;

#[path = "../service/services.rs"]
mod services;

#[path = "../service/shared/enums.rs"]
mod enums;

pub fn main() -> Result<()> {
    let mut root = env::current_dir()?;
    loop {
        if root.join(".cargo").exists() {
            break;
        }
        root = root.parent().unwrap().to_owned();
    }
    let root = root.to_str().unwrap();
    let dir = format!("{}/src/gen", root);
    create_dir_all(&dir)?;
    docs::gen_services_docs(root)?;
    docs::gen_md_docs(root)?;
    rust::gen_model_rs(root, &dir)?;
    sql::gen_model_sql(root)?;
    sql::gen_db_sql(root)?;
    rust::gen_db_rs(&dir)?;
    docs::gen_systemd_services(root, "mc2fi", "mc2fi")?;
    docs::gen_error_message_md(root)?;
    Ok(())
}
