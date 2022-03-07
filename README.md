# Compute benchmarks

This repo is designed to show/measure different compute unit benchmarks for common solana tasks.

See the article about it here: [TDB]

## Addition template for testing anchor apps under Github Actions

I couldn't find a good example project of testing anchor programs with Github Actions.

In my case, I wanted the following:

- Specific version of Anchor installed
- Ability to cache the Anchor install (reduce test setup time from 10's of minutes to seconds)
- Cache node_modules

## Install and run

Assumes you have 'anchor' installed and set up.

1. `yarn install`
2. `anchor test`