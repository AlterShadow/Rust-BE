use model::endpoint::*;
use model::types::{Field, Type};

pub fn endpoint_user_transfer_organization_owner() -> EndpointSchema {
    EndpointSchema::new(
        "TransferOrganizationOwner",
        20040,
        vec![
            Field::new("organization_id", Type::BigInt),
            Field::new("transfer_organization_owner_key", Type::String),
            Field::new("new_owner_user_id", Type::BigInt),
        ],
        vec![],
    )
}

pub fn endpoint_user_list_organization_membership() -> EndpointSchema {
    EndpointSchema::new(
        "ListOrganizationMembership",
        20042,
        vec![],
        vec![Field::new(
            "memberships",
            Type::data_table(
                "ListOrganizationMembershipRow",
                vec![
                    Field::new("organization_id", Type::BigInt),
                    Field::new("organization_name", Type::String),
                    Field::new("user_id", Type::BigInt),
                    Field::new("role", Type::enum_ref("role")),
                    Field::new("accepted", Type::Boolean),
                ],
            ),
        )],
    )
}

pub fn get_user_endpoints() -> Vec<EndpointSchema> {
    vec![
        endpoint_user_transfer_organization_owner(),
        endpoint_user_list_organization_membership(),
    ]
}
