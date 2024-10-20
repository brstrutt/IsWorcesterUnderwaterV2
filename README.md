# IsWorcesterUnderwaterV2

![GitHub Last Commit](https://img.shields.io/github/last-commit/brstrutt/IsWorcesterUnderwaterV2?logo=github)
![Build and Publish](https://github.com/brstrutt/IsWorcesterUnderwaterV2/actions/workflows/build-and-publish.yml/badge.svg?branch=main)

A rewrite of [isworcesterunderwater.co.uk](https://isworcesterunderwater.co.uk) to be a static site built using Rust and Yew.

## Development

1. Clone the repo
2. Open the devcontainer in vscode
3. To see the site run `trunk serve`. This will serve the site on `http://127.0.0.1:8080/IsWorcesterUnderwaterV2`. This will also auto rebuild and refresh the page when you save any changes
4. To build a release run `trunk build --release`. The site will be built in the *./dist* directory

## Publishing

1. Push to main
2. Github actions should build the site and push to production automatically

## Tools

- `devcontainer`: docker based development environment with lots of vscode integration.
- `Rust`: programming language
- `Cargo`: standard tool for working with `Rust`. Used for building/test/managing dependencies
- `Yew`: Frontend library for developing webapps in `Rust`. Think `React` but for `Rust`
- `Trunk`: Tool to make developing webapps much easier in `Rust`. Use this instead of `Cargo` for building/running the project.

## Previous version

Original implementation can be found [here](https://github.com/brstrutt/IsWorcesterUnderwater)
