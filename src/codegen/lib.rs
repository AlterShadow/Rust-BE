pub mod rust;
pub mod service;
pub mod sql;

use crate::rust::{to_rust_decl, to_rust_type_decl, ToRust};
use crate::service::get_systemd_service;
use crate::sql::ToSql;
use convert_case::{Case, Casing};
use eyre::*;
use itertools::Itertools;
use model::service::Service;
use model::types::*;
use serde::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::Command;

pub const SYMBOL: &str = "a_";
#[path = "../service/services.rs"]
mod services;

#[path = "../service/enums.rs"]
mod enums;
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
pub fn collect_rust_recursive_types(t: Type) -> Vec<Type> {
    match t {
        Type::Object { ref fields, .. } => {
            let mut v = vec![t.clone()];
            for x in fields {
                v.extend(collect_rust_recursive_types(x.ty.clone()));
            }
            v
        }
        Type::DataTable { name, fields } => {
            collect_rust_recursive_types(Type::object(name, fields))
        }
        Type::Vec(x) => collect_rust_recursive_types(*x),
        _ => vec![],
    }
}

pub fn check_endpoint_codes() -> Result<()> {
    let mut codes = HashMap::new();
    for s in services::get_services() {
        for e in s.endpoints {
            let code = e.code;
            if codes.contains_key(&code) {
                bail!("duplicate service code: {} {} {}", s.name, e.name, e.code);
            }
            codes.insert(code, e.code);
        }
    }
    Ok(())
}

pub fn gen_model_rs(root: &str, dir: &str) -> Result<()> {
    let db_filename = format!("{}/model.rs", dir);
    let mut f = File::create(&db_filename)?;

    write!(
        &mut f,
        "{}",
        r#"
use tokio_postgres::types::*;
use serde::*;
use num_derive::FromPrimitive;
use strum_macros::EnumString;
use lib::error_code::ErrorCode;
    "#
    )?;

    for e in enums::get_enums() {
        writeln!(&mut f, "{}", e.to_rust_decl())?;
    }

    let errors = get_error_messages(root)?;
    let rule = regex::Regex::new(r"\{[\w]+}")?;

    for e in &errors.codes {
        let name = format!("Error{}", e.symbol.to_case(Case::Pascal));
        let s = Type::object(
            name,
            rule.find_iter(&e.message)
                .map(|m| m.as_str())
                .map(|s| s.trim_matches('{').trim_matches('}'))
                .map(|s| Field::new(s.to_string(), Type::String))
                .collect(),
        );
        writeln!(
            &mut f,
            r#"#[derive(Serialize, Deserialize, Debug)]
               #[serde(rename_all = "camelCase")]
               {}"#,
            s.to_rust_decl()
        )?;
    }
    let enum_ = Type::enum_(
        "ErrorCode",
        errors
            .codes
            .into_iter()
            .map(|x| {
                EnumVariant::new_with_comment(
                    x.symbol.to_case(Case::Pascal),
                    x.code,
                    format!("{} {}", x.source, x.message),
                )
            })
            .collect(),
    );
    writeln!(&mut f, "{}", enum_.to_rust_decl())?;
    writeln!(
        &mut f,
        r#"
impl Into<ErrorCode> for EnumErrorCode {{
    fn into(self) -> ErrorCode {{
        ErrorCode::new(self as _)
    }}
}}
    "#
    )?;

    let mut types = HashSet::new();
    for s in services::get_services() {
        for e in s.endpoints {
            let req = Type::object(format!("{}Request", e.name), e.parameters);
            let resp = Type::object(format!("{}Response", e.name), e.returns);
            types.extend(
                vec![
                    collect_rust_recursive_types(req),
                    collect_rust_recursive_types(resp),
                ]
                .concat()
                .into_iter(),
            );
        }
    }
    for s in types {
        write!(
            &mut f,
            r#"#[derive(Serialize, Deserialize, Debug, Clone)]
                    #[serde(rename_all = "camelCase")]
                    {}"#,
            s.to_rust_decl()
        )?;
    }
    f.flush()?;
    drop(f);
    rustfmt(&db_filename)?;

    Ok(())
}
pub fn gen_model_sql(root: &str) -> Result<()> {
    let db_filename = format!("{}/db/model.sql", root);
    let mut f = File::create(db_filename)?;

    for e in enums::get_enums() {
        match e {
            Type::Enum { name, variants } => {
                writeln!(
                    &mut f,
                    "CREATE TYPE enum_{} AS ENUM ({});",
                    name,
                    variants
                        .into_iter()
                        .map(|x| format!("'{}'", x.name))
                        .join(", ")
                )?;
            }
            _ => unreachable!(),
        }
    }
    f.flush()?;
    drop(f);
    Ok(())
}
pub fn rustfmt(f: &str) -> Result<()> {
    let exit = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(f)
        .spawn()?
        .wait()?;
    if !exit.success() {
        bail!("failed to rustfmt {:?}", exit);
    }
    Ok(())
}
pub fn gen_db_rs(dir: &str) -> Result<()> {
    let funcs = services::get_proc_functions();

    let db_filename = format!("{}/database.rs", dir);
    let mut db = File::create(&db_filename)?;

    write!(
        &mut db,
        "{}",
        r#"
use eyre::*;
use lib::database::*;
use crate::model::*;
use serde::*;

#[derive(Clone)]
pub struct DbClient {
    pub client: SimpleDbClient
}
impl DbClient {
    pub fn new(client: SimpleDbClient) -> Self {
        Self {
            client
        }
    }
}
impl From<SimpleDbClient> for DbClient {
    fn from(client: SimpleDbClient) -> Self {
        Self::new(client)
    }
}
    "#
    )?;
    for func in funcs {
        write!(
            &mut db,
            "
{}
impl DbClient {{ 
    #[allow(unused_variables)]
    {}
}}",
            to_rust_type_decl(&func),
            to_rust_decl(&func)
        )?;
    }
    db.flush()?;
    drop(db);
    rustfmt(&db_filename)?;
    Ok(())
}

pub fn gen_client_rs(dir: &str) -> Result<()> {
    let services = services::get_services();

    let rs_filename = format!("{}/client.rs", dir);
    let mut rs = File::create(&rs_filename)?;
    write!(
        &mut rs,
        "{}",
        r#"
use eyre::*;
use lib::ws::WsClient;
use crate::model::*;
    "#
    )?;
    for s in services {
        write!(
            &mut rs,
            r#"
pub struct {srv_name}Client {{
    pub client: WsClient
}}
impl {srv_name}Client {{
    pub fn new(client: WsClient) -> Self {{
        Self {{
            client
        }}
    }}
}}
impl From<WsClient> for {srv_name}Client {{
    fn from(client: WsClient) -> Self {{
        Self::new(client)
    }}
}}
    "#,
            srv_name = s.name.to_case(Case::Pascal)
        )?;

        for endpoint in s.endpoints {
            write!(
                &mut rs,
                "
impl {srv_name}Client {{
    pub async fn {end_name}(&mut self, req: {end_name2}Request) -> Result<{end_name2}Response> {{
        self.client.request({code}, req).await
    }}
}}",
                srv_name = s.name.to_case(Case::Pascal),
                end_name = endpoint.name.to_case(Case::Snake),
                end_name2 = endpoint.name.to_case(Case::Pascal),
                code = endpoint.code
            )?;
        }
    }
    rs.flush()?;
    drop(rs);
    rustfmt(&rs_filename)?;
    Ok(())
}

pub fn gen_db_sql(root: &str) -> Result<()> {
    let funcs = services::get_proc_functions();

    let db_filename = format!("{}/db/api.sql", root);
    let mut f = File::create(&db_filename)?;
    writeln!(&mut f, "{}", r#"CREATE SCHEMA IF NOT EXISTS api;"#)?;
    for func in funcs {
        writeln!(&mut f, "{}", func.to_sql())?;
    }
    for srv in services::get_services() {
        writeln!(
            &mut f,
            "{}",
            ProceduralFunction::new(
                format!("{}_SERVICE", srv.name.to_case(Case::ScreamingSnake)),
                vec![],
                vec![Field::new("code", Type::Int)],
                format!("BEGIN RETURN QUERY SELECT {}; END", srv.id),
            )
            .to_sql()
        )?;
    }
    f.flush()?;
    drop(f);

    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
struct Docs {
    services: Vec<Service>,
    enums: Vec<Type>,
}
pub fn gen_services_docs(root: &str) -> Result<()> {
    let docs = Docs {
        services: services::get_services(),
        enums: enums::get_enums(),
    };
    let docs_filename = format!("{}/docs/services.json", root);
    let mut docs_file = File::create(docs_filename)?;
    serde_json::to_writer_pretty(&mut docs_file, &docs)?;
    Ok(())
}

pub fn gen_md_docs(root: &str) -> Result<()> {
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
pub fn gen_systemd_services(root: &str, app_name: &str, user: &str) -> Result<()> {
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

pub fn get_error_messages(root: &str) -> Result<ErrorMessages> {
    let def_filename = format!("{}/docs/error_codes/error_codes_en.json", root);
    let def_file = std::fs::read(def_filename)?;
    let definitions: ErrorMessages = serde_json::from_slice(&def_file)?;
    Ok(definitions)
}
pub fn gen_error_message_md(root: &str) -> Result<()> {
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
pub fn main() -> Result<()> {
    check_endpoint_codes()?;
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
    gen_services_docs(root)?;
    gen_md_docs(root)?;
    gen_model_rs(root, &dir)?;
    gen_model_sql(root)?;
    gen_db_sql(root)?;
    gen_client_rs(&dir)?;
    gen_db_rs(&dir)?;
    gen_systemd_services(root, "mc2fi", "mc2fi")?;
    gen_error_message_md(root)?;
    Ok(())
}
