use crate::{enums, services};
use convert_case::{Case, Casing};
use itertools::Itertools;
use model::types::*;
use std::fs::File;
use std::io::Write;
pub const PARAM_PREFIX: &str = "a_";

pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for Type {
    fn to_sql(&self) -> String {
        match self {
            Type::Date => "int".to_owned(), // TODO: fix things
            Type::Int => "int".to_owned(),
            Type::BigInt => "bigint".to_owned(),
            Type::Numeric => "double precision".to_owned(),
            Type::Struct { fields, .. } => {
                let fields = fields
                    .iter()
                    .map(|x| format!("\"{}\" {}", x.name, x.ty.to_sql()));
                format!(
                    "table (\n{}\n)",
                    fields.map(|x| format!("    {}", x)).join(",\n")
                )
            }
            Type::StructRef(_name) => "jsonb".to_owned(),
            Type::Object => "jsonb".to_owned(),
            Type::DataTable { .. } => {
                todo!()
            }
            Type::Vec(fields) => {
                format!("{}[]", fields.to_sql())
            }
            Type::Unit => "void".to_owned(),
            Type::Optional(t) => format!("{}", t.to_sql()),
            Type::Boolean => "boolean".to_owned(),
            Type::String => "varchar".to_owned(),
            Type::Bytea => "bytea".to_owned(),
            Type::UUID => "uuid".to_owned(),
            Type::Inet => "inet".to_owned(),
            Type::Enum { name, .. } => format!("enum_{}", name),
            Type::EnumRef(name) => format!("enum_{}", name),
        }
    }
}
impl ToSql for ProceduralFunction {
    fn to_sql(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|x| match &x.ty {
                Type::Optional(y) => {
                    format!("{}{} {} DEFAULT NULL", PARAM_PREFIX, x.name, y.to_sql())
                }
                y => format!("{}{} {}", PARAM_PREFIX, x.name, y.to_sql()),
            })
            .join(", ");
        let returns = if self.returns.len() == 0 {
            "void".to_owned()
        } else {
            Type::struct_("", self.returns.clone()).to_sql()
        };
        format!(
            "
CREATE OR REPLACE FUNCTION api.{name}({params})
RETURNS {returns}
LANGUAGE plpgsql
AS $$
    {body}
$$;
        ",
            name = self.name,
            params = params,
            returns = returns,
            body = self.body.replace("$", PARAM_PREFIX)
        )
    }
}

pub fn gen_db_sql(root: &str) -> eyre::Result<()> {
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

pub fn gen_model_sql(root: &str) -> eyre::Result<()> {
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
