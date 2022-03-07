# Solana Computes

This repo serves two purposes at the moment:

1. To demonstrate how to easily benchmark compute units of common Solana instructions
1. As a template for .github actions on an Anchor project


## Current benchmarks

Only demonstrating some basic uses here, but in this case we look at the cost of creating a Pubkey from a
base58 string, versus using the 'pubkey!' macro.

Test Output: 

```
  compute_unit_benchmark
Baseline compute units:  524
Pubkey from macro used 12335 of compute units (+11811)
Pubkey from string used 20999 of compute units (+20475)
    ✔ It should measure a benchmark for pubkey parsing (1720ms)
```

## Anchor Github Actions

I couldn't find a good example project for testing anchor programs with Github Actions.

In my case, I wanted the following:

- User can specify the version of Anchor installed
- Ability to cache the Anchor install (reduce test setup time from 10's of minutes to under 2 minutes)
- Cache node_modules

The process can be summed up as:

1. Setup base linux install with necessary build packages
1. Setup Solana and Anchor CLI
1. Setup Node.js
1. Run 'yarn' to install javascript dependecies
1. Build and test anchor project

## Install and run locally

Assumes you have 'anchor' correctly installed and set up.

1. `cd this_project_dir`
1. `yarn`
1. `anchor test`