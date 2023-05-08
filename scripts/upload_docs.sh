#!/bin/bash
set -e

FRONTEND=../app.mc2.fi-frontend
(cd "$FRONTEND" && git pull)
rsync -avizh docs/. "$FRONTEND"/docs
(cd "$FRONTEND" && git add docs && git commit -m "[feat] Update docs" && git push)
