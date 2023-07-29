#!/bin/bash
systemctl daemon-reload
systemctl restart mc2fi_auth
systemctl restart mc2fi_user
systemctl restart mc2fi_admin
systemctl restart mc2fi_watcher
systemctl restart mc2fi_watcher_test
systemctl restart mc2fi_asset_price


