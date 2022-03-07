#!/bin/bash

if command -v 'anchor'; then
    ANCHOR_INSTALLED_VERSION=$(anchor --version | awk '{print $NF}')
    echo "Installed anchor version at '$ANCHOR_INSTALLED_VERSION', require '$ANCHOR_VERSION'"

    if [[ $ANCHOR_INSTALLED_VERSION != $ANCHOR_VERSION ]]; then
        echo "Installing v$ANCHOR_VERSION"
        cargo install --git https://github.com/project-serum/anchor --tag v${{ ANCHOR_VERSION }} anchor-cli --locked --force; \
    else
        echo "Anchor up to date"
    fi

else

    echo "Didn't find a version of anchor installed, installing anchor v$ANCHOR_VERSION"
    cargo install --git https://github.com/project-serum/anchor --tag v${{ ANCHOR_VERSION }} anchor-cli --locked --force; \

fi
