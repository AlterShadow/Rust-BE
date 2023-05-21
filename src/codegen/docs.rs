use crate::service::get_systemd_service;
use crate::{enums, services};
use itertools::Itertools;
use model::service::Service;
use model::types::Type;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Docs {
    services: Vec<Service>,
    enums: Vec<Type>,
}

pub fn gen_services_docs(root: &str) -> eyre::Result<()> {
    let docs = Docs {
        services: services::get_services(),
        enums: enums::get_enums(),
    };
    let docs_filename = format!("{}/docs/services.json", root);
    let mut docs_file = File::create(docs_filename)?;
    serde_json::to_writer_pretty(&mut docs_file, &docs)?;
    Ok(())
}

pub fn gen_md_docs(root: &str) -> eyre::Result<()> {
    let docs_filename = format!("{}/docs/README.md", root);
    let mut docs_file = File::create(docs_filename)?;
    for s in services::get_services() {
        writeln!(
            &mut docs_file,
            r#"
# {} Server
ID: {}
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|"#,
            s.name, s.id
        )?;
        for e in s.endpoints {
            writeln!(
                &mut docs_file,
                "|{}|{}|{}|{}|{}|",
                e.code,
                e.name,
                e.parameters
                    .iter()
                    .map(|x| format!("{}", x.name))
                    .join(", "),
                e.returns.iter().map(|x| format!("{}", x.name)).join(", "),
                e.description
            )?;
        }
    }
    Ok(())
}

pub fn gen_systemd_services(root: &str, app_name: &str, user: &str) -> eyre::Result<()> {
    create_dir_all(format!("{}/etc/systemd", root))?;
    let services = services::get_services();
    for srv in services {
        let service_filename = format!("{}/etc/systemd/{}_{}.service", root, app_name, srv.name);
        let mut service_file = File::create(&service_filename)?;
        let v = get_systemd_service(app_name, &srv.name, user);
        write!(&mut service_file, "{}", v)?;
    }
    Ok(())
}

pub fn get_error_messages(root: &str) -> eyre::Result<ErrorMessages> {
    let def_filename = format!("{}/docs/error_codes/error_codes_en.json", root);
    let def_file = std::fs::read(def_filename)?;
    let definitions: ErrorMessages = serde_json::from_slice(&def_file)?;
    Ok(definitions)
}

pub fn gen_error_message_md(root: &str) -> eyre::Result<()> {
    let definitions = get_error_messages(root)?;
    let doc_filename = format!("{}/docs/error_codes/error_codes.md", root);
    let mut doc_file = File::create(doc_filename)?;
    writeln!(
        &mut doc_file,
        r#"
# Error Messages
|Error Code|Error Symbol|Error Message|Error Source|
|----------|------------|-------------|------------|"#,
    )?;
    for item in definitions.codes {
        writeln!(
            &mut doc_file,
            "|{}|{}|{}|{}|",
            item.code, item.symbol, item.message, item.source
        )?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessages {
    pub language: String,
    pub codes: Vec<ErrorMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub code: i64,
    #[serde(default)]
    pub symbol: String,
    pub message: String,
    #[serde(default)]
    pub source: String,
}
