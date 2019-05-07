[![pipeline status](https://gitlab.com/mateuscosta/ranbumfy/badges/master/pipeline.svg)](https://gitlab.com/mateuscosta/ranbumfy/commits/master)
[![Build Status](https://travis-ci.com/mrcosta/ranbumfy.svg?branch=master)](https://travis-ci.com/mrcosta/ranbumfy)
[![codecov](https://codecov.io/gh/mrcosta/ranbumfy/branch/master/graph/badge.svg)](https://codecov.io/gh/mrcosta/ranbumfy)

randomize `n` albums to be listen based in your followed artists

# Executing

* create an `.env` file in the root of the project like this:
```.env
CLIENT_ID="somevaluefromspotify"
CLIENT_SECRET="somevaluefromspotify"
REDIRECT_URI="http://localhost"
```
* to run: `cargo run -- -w`
* or with the binary: `./target/debug/ranbumfy -w`

# contributing

* to run tests: `cargo test`
* to run only integration tests: `cargo test --test integration`
* to run only one test: `cargo test <TESTNAME>`
* to run only unit tests: `cargo test --bin ranbumfy`
* take a look in `.gitlab-ci.yml` to see which rust version is being supported
