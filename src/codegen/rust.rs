use crate::sql::{ToSql, PARAM_PREFIX};
use crate::{docs, enums, services};
use convert_case::{Case, Casing};
use eyre::bail;
use itertools::Itertools;
use model::pg_func::ProceduralFunction;
use model::types::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub trait ToRust {
    fn to_rust_ref(&self, serde_with: bool) -> String;
    fn to_rust_decl(&self, serde_with: bool) -> String;
}

impl ToRust for Type {
    fn to_rust_ref(&self, serde_with: bool) -> String {
        match self {
            Type::Date => "u32".to_owned(), // TODO: resolve date
            Type::Int => "i32".to_owned(),
            Type::BigInt => "i64".to_owned(),
            Type::Numeric => "f64".to_owned(),
            Type::Struct { name, .. } => name.clone(),
            Type::StructRef(name) => name.clone(),
            Type::Object => "serde_json::Value".to_owned(),
            Type::DataTable { name, .. } => format!("Vec<{}>", name),
            Type::Vec(ele) => {
                format!("Vec<{}>", ele.to_rust_ref(serde_with))
            }
            Type::Unit => "()".to_owned(),
            Type::Optional(t) => {
                format!("Option<{}>", t.to_rust_ref(serde_with))
            }
            Type::Boolean => "bool".to_owned(),
            Type::String => "String".to_owned(),
            Type::Bytea => "Vec<u8>".to_owned(),
            Type::UUID => "uuid::Uuid".to_owned(),
            Type::Inet => "std::net::IpAddr".to_owned(),
            Type::Enum { name, .. } => format!("Enum{}", name.to_case(Case::Pascal),),
            Type::EnumRef(name) => format!("Enum{}", name.to_case(Case::Pascal),),
            Type::BlockchainDecimal if serde_with => "U256".to_owned(),
            Type::BlockchainAddress if serde_with => "Address".to_owned(),
            Type::BlockchainTransactionHash if serde_with => "H256".to_owned(),
            Type::BlockchainDecimal => "BlockchainDecimal".to_owned(),
            Type::BlockchainAddress => "BlockchainAddress".to_owned(),
            Type::BlockchainTransactionHash => "BlockchainTransactionHash".to_owned(),
        }
    }

    fn to_rust_decl(&self, serde_with: bool) -> String {
        match self {
            Type::Struct { name, fields } => {
                let mut fields = fields.iter().map(|x| {
                    let opt = matches!(&x.ty, Type::Optional(_));
                    let serde_with_opt = match &x.ty {
                        Type::BlockchainDecimal if serde_with => "WithBlockchainDecimal",
                        Type::BlockchainAddress if serde_with => "WithBlockchainAddress",
                        Type::BlockchainTransactionHash if serde_with => {
                            "WithBlockchainTransactionHash"
                        }
                        _ => "",
                    };
                    format!(
                        "{} {} pub {}: {}",
                        if opt { "#[serde(default)]" } else { "" },
                        if serde_with_opt.is_empty() {
                            "".to_string()
                        } else {
                            format!("#[serde(with = \"{}\")]", serde_with_opt)
                        },
                        x.name,
                        x.ty.to_rust_ref(serde_with)
                    )
                });
                format!("pub struct {} {{{}}}", name, fields.join(","))
            }
            Type::Enum {
                name,
                variants: fields,
            } => {
                let mut fields = fields.iter().map(|x| {
                    format!(
                        r#"
    /// {}
    #[postgres(name = "{}")]
    {} = {}
"#,
                        x.comment,
                        x.name,
                        if x.name.chars().last().unwrap().is_lowercase() {
                            x.name.to_case(Case::Pascal)
                        } else {
                            x.name.clone()
                        },
                        x.value
                    )
                });
                format!(
                    r#"#[derive(Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, Hash)] #[postgres(name = "enum_{}")]pub enum Enum{} {{{}}}"#,
                    name,
                    name.to_case(Case::Pascal),
                    fields.join(",")
                )
            }
            x => x.to_rust_ref(serde_with),
        }
    }
}

pub fn get_parameter_type(this: &ProceduralFunction) -> Type {
    Type::struct_(
        format!("{}Req", this.name.to_case(Case::Pascal)),
        this.parameters.clone(),
    )
}

pub fn pg_func_to_rust_trait_impl(this: &ProceduralFunction) -> String {
    let mut arguments = this.parameters.iter().enumerate().map(|(i, x)| {
        format!(
            "{}{} => ${}::{}",
            PARAM_PREFIX,
            x.name,
            i + 1,
            x.ty.to_sql()
        )
    });
    let sql = format!("SELECT * FROM api.{}({});", this.name, arguments.join(", "));
    let pg_params = this
        .parameters
        .iter()
        .map(|x| format!("&self.{} as &(dyn ToSql + Sync)", x.name))
        .join(", ");

    format!(
        "
        #[allow(unused_variables)]
        impl DatabaseRequest for {name}Req {{
          type ResponseRow = {ret_name};
          fn statement(&self) -> &str {{
            \"{sql}\"
          }}
          fn params(&self) -> Vec<&(dyn ToSql + Sync)> {{
            vec![{pg_params}]
          }}
        }}
",
        name = this.name.to_case(Case::Pascal),
        ret_name = match &this.return_row_type {
            Type::Struct { name, .. } => name,
            _ => unreachable!(),
        },
        sql = sql,
        pg_params = pg_params,
    )
}

pub fn gen_db_rs(dir: &str) -> eyre::Result<()> {
    let funcs = services::get_proc_functions();

    let db_filename = format!("{}/database.rs", dir);
    let mut db = File::create(&db_filename)?;

    write!(
        &mut db,
        "{}",
        r#"
use lib::database::*;
use lib::types::*;
use crate::model::*;
use serde::*;
use postgres_from_row::FromRow;

    "#
    )?;
    let mut types = BTreeSet::new();
    for func in &funcs {
        types.insert(&func.return_row_type);
    }
    for ty in types {
        write!(
            &mut db,
            "
{}
",
            [ty].into_iter()
                .filter(|x| !matches!(x, Type::Unit))
                .map(|x| {
                    format!(
                        "#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]\n{}",
                        x.to_rust_decl(false)
                    )
                })
                .join("\n"),
        )?;
    }
    for func in funcs {
        write!(
            &mut db,
            "
{}
{}
",
            [get_parameter_type(&func)]
                .into_iter()
                .filter(|x| !matches!(x, Type::Unit))
                .map(|x| {
                    format!(
                        "#[derive(Serialize, Deserialize, Debug, Clone)]\n{}",
                        x.to_rust_decl(false)
                    )
                })
                .join("\n"),
            pg_func_to_rust_trait_impl(&func)
        )?;
    }
    db.flush()?;
    drop(db);
    rustfmt(&db_filename)?;
    Ok(())
}

pub fn collect_rust_recursive_types(t: Type) -> Vec<Type> {
    match t {
        Type::Struct { ref fields, .. } => {
            let mut v = vec![t.clone()];
            for x in fields {
                v.extend(collect_rust_recursive_types(x.ty.clone()));
            }
            v
        }
        Type::DataTable { name, fields } => {
            collect_rust_recursive_types(Type::struct_(name, fields))
        }
        Type::Vec(x) => collect_rust_recursive_types(*x),
        Type::Optional(x) => collect_rust_recursive_types(*x),
        _ => vec![],
    }
}

pub fn gen_model_rs(root: &str, dir: &str) -> eyre::Result<()> {
    let db_filename = format!("{}/model.rs", dir);
    let mut f = File::create(&db_filename)?;
    write!(
        &mut f,
        "{}",
        r#"
use tokio_postgres::types::*;
use serde::*;
use num_derive::FromPrimitive;
use strum_macros::{EnumString, Display};
use lib::error_code::ErrorCode;
use lib::ws::*;
use lib::types::*;

    "#
    )?;

    for e in enums::get_enums() {
        writeln!(&mut f, "{}", e.to_rust_decl(false))?;
    }
    check_endpoint_codes(&mut f)?;

    let errors = docs::get_error_messages(root)?;
    let rule = regex::Regex::new(r"\{[\w]+}")?;

    for e in &errors.codes {
        let name = format!("Error{}", e.symbol.to_case(Case::Pascal));
        let s = Type::struct_(
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
            s.to_rust_decl(true)
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
    writeln!(&mut f, "{}", enum_.to_rust_decl(false))?;
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

    let mut types = BTreeSet::new();
    for s in services::get_services() {
        for e in s.endpoints {
            let req = Type::struct_(format!("{}Request", e.name), e.parameters);
            let resp = Type::struct_(format!("{}Response", e.name), e.returns);
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
            s.to_rust_decl(true)
        )?;
    }

    for s in services::get_services() {
        for endpoint in s.endpoints {
            write!(
                &mut f,
                "
impl WsRequest for {end_name2}Request {{
    type Response = {end_name2}Response;
    const METHOD_ID: u32 = {code};
    const SCHEMA: &'static str = r#\"{schema}\"#;
}}
impl WsResponse for {end_name2}Response {{
    type Request = {end_name2}Request;
}}
",
                end_name2 = endpoint.name.to_case(Case::Pascal),
                code = endpoint.code,
                schema = serde_json::to_string_pretty(&endpoint).unwrap()
            )?;
        }
    }
    f.flush()?;
    drop(f);
    rustfmt(&db_filename)?;

    Ok(())
}

pub fn rustfmt(f: &str) -> eyre::Result<()> {
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

pub fn check_endpoint_codes(mut writer: impl Write) -> eyre::Result<()> {
    let mut variants = vec![];
    for s in services::get_services() {
        for e in s.endpoints {
            variants.push(EnumVariant::new(e.name, e.code as _));
        }
    }
    let enum_ = Type::enum_("Endpoint", variants);
    writeln!(writer, "{}", enum_.to_rust_decl(false))?;
    // if it compiles, there're no duplicate codes or names
    Ok(())
}
