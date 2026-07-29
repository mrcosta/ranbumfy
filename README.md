[![pipeline status](https://gitlab.com/mateuscosta/ranbumfy/badges/master/pipeline.svg)](https://gitlab.com/mateuscosta/ranbumfy/commits/master)
[![Build Status](https://travis-ci.com/mrcosta/ranbumfy.svg?branch=master)](https://travis-ci.com/mrcosta/ranbumfy)
[![codecov](https://codecov.io/gh/mrcosta/ranbumfy/branch/master/graph/badge.svg)](https://codecov.io/gh/mrcosta/ranbumfy)

randomize `n` albums to be listen based in your followed artists

# Executing

* create an `.env` file in the root of the project like this:
```.env
RSPOTIFY_CLIENT_ID="get me in https://developer.spotify.com/dashboard/"
RSPOTIFY_CLIENT_SECRET="get me in https://developer.spotify.com/dashboard/"
RSPOTIFY_REDIRECT_URI="http://127.0.0.1:8888/callback"
```
* Make sure the exact same `http://127.0.0.1:8888/callback` is added to your Spotify app's Redirect URIs in the dashboard.
  Spotify rejects `http://localhost` with `redirect_uri: Insecure`: plain HTTP is only allowed on the loopback IP `127.0.0.1`, anything else must be HTTPS
* nothing listens on that port, so the browser will show a "connection refused" page after you authorize.
  That is expected: copy the full URL from the address bar (`http://127.0.0.1:8888/callback?code=...`) and paste it into the prompt
* to run: `cargo run -- -w`
* or with the binary: `./target/debug/ranbumfy -w`

# contributing

* to run tests: `cargo test`
* to run only integration tests: `cargo test --test integration`
* to run only one test: `cargo test <TESTNAME>`
* to run only unit tests: `cargo test --bin ranbumfy`
* take a look in `.gitlab-ci.yml` to see which rust version is being supported
