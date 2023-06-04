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
        Type::enum_(
            "block_chain".to_owned(),
            vec![
                EnumVariant::new("EthereumMainnet", 0),
                EnumVariant::new("EthereumGoerli", 1),
                EnumVariant::new("BscMainnet", 2),
                EnumVariant::new("BscTestnet", 3),
                EnumVariant::new("LocalNet", 4),
            ],
        ),
        Type::enum_(
            "blockchain_coin".to_owned(),
            vec![
                EnumVariant::new("USDC", 0),
                EnumVariant::new("USDT", 1),
                EnumVariant::new("BUSD", 2),
            ],
        ),
        Type::enum_(
            "dex".to_owned(),
            vec![
                EnumVariant::new("UniSwap", 0),
                EnumVariant::new("PancakeSwap", 1),
                EnumVariant::new("SushiSwap", 2),
            ],
        ),
        Type::enum_(
            "dex_version".to_owned(),
            vec![
                EnumVariant::new("V1", 0),
                EnumVariant::new("V2", 1),
                EnumVariant::new("V3", 2),
            ],
        ),
        get_service_enum(),
    ]
}
