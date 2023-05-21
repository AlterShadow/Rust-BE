#!/bin/bash
systemctl daemon-reload
systemctl restart mc2fi_auth
systemctl restart mc2fi_user
systemctl restart mc2fi_admin
systemctl restart mc2fi_trade-watcher
systemctl restart mc2fi_escrow-watcher


