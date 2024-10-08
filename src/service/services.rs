use model::pg_func::ProceduralFunction;
use model::service::*;

#[path = "auth/endpoints.rs"]
mod auth_endpoints;
#[path = "auth/pg_func.rs"]
mod auth_pg_func;

#[path = "user/endpoints.rs"]
mod user_endpoints;

#[path = "user/pg_func.rs"]
mod user_pg_func;

#[path = "admin/endpoints.rs"]
mod admin_endpoints;

#[path = "admin/pg_func.rs"]
mod admin_pg_func;
#[path = "asset_price/pg_func.rs"]
mod asset_price_pg_func;
#[path = "watcher/pg_func.rs"]
mod watcher_pg_func;

pub fn get_services() -> Vec<Service> {
    vec![
        Service::new("auth", 1, auth_endpoints::get_auth_endpoints()),
        Service::new("user", 2, user_endpoints::get_user_endpoints()),
        Service::new("admin", 3, admin_endpoints::get_admin_endpoints()),
        Service::new("watcher", 4, vec![]),
        Service::new("asset_price", 5, vec![]),
    ]
}

pub fn get_proc_functions() -> Vec<ProceduralFunction> {
    vec![
        auth_pg_func::get_auth_pg_func(),
        user_pg_func::get_user_pg_func(),
        admin_pg_func::get_admin_pg_func(),
        watcher_pg_func::get_trade_watcher_pg_func(),
        asset_price_pg_func::get_asset_price_pg_func(),
    ]
    .concat()
}
