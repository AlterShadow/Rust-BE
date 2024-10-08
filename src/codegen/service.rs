pub fn get_systemd_service(app_name: &str, service_name: &str, user: &str) -> String {
    format!(
        r#"[Unit]
Description={app_name} {service_name}
After=network.target
StartLimitIntervalSec=0

[Service]
User={user}
Type=simple
Restart=always
RestartSec=1
WorkingDirectory=/home/{user}/{app_name}
ExecStart=/home/{user}/{app_name}/target/release/{app_name}_{service_name} --config=etc/config.json

StandardError=append:/home/{user}/{app_name}/log/{app_name}_{service_name}.log
StandardOutput=append:/home/{user}/{app_name}/log/{app_name}_{service_name}.log
StandardInput=null
AmbientCapabilities=CAP_NET_BIND_SERVICE

[Install]
WantedBy=default.target

"#,
        app_name = app_name,
        service_name = service_name,
        user = user
    )
}
