use crate::services::get_services;
use model::types::*;

pub fn get_service_enum() -> Type {
    Type::enum_(
        "service".to_owned(),
        get_services()
            .iter()
            .map(|s| EnumVariant::new(s.name.clone(), s.id as _))
            .collect::<Vec<EnumVariant>>(),
    )
}
pub fn get_enums() -> Vec<Type> {
    vec![
        Type::enum_(
            "role".to_owned(),
            vec![
                EnumVariant::new("guest", 0),
                EnumVariant::new("user", 1),
                EnumVariant::new("admin", 2),
                EnumVariant::new("expert", 3),
                EnumVariant::new("developer", 4),
            ],
        ),
        get_service_enum(),
    ]
}
