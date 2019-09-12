#!/bin/bash

set +e

CONFIGS_DIR="${CONFIGS_DIR:-../../configs}"
APPS_DIR="${APPS_DIR:-../../apps}"


if hash cargo 2>/dev/null; then
    cargo build --release

    cp -f target/release/cnoise ${APPS_DIR}
    cp -f target/release/bimood ${APPS_DIR}
    cp -f target/release/predprey ${APPS_DIR}
    cp -f target/release/maze ${APPS_DIR}
    cp -f target/release/dualmaze ${APPS_DIR}

    cp -f pixelfooconf.py ${CONFIGS_DIR}
fi
