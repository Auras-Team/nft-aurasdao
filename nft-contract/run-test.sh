#!/bin/bash
set -e && RUSTFLAGS='-C link-arg=-s' cargo test

