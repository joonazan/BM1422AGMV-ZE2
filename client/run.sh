#!/usr/bin/env bash

python bluetooth_receiver.py | $(cd haarukointi && cargo run --release)
