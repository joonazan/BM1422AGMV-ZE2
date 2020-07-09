#!/usr/bin/env bash

python3 bluetooth_receiver.py | $(cd haarukointi && cargo run --release)
