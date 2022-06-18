#!/bin/sh

cargo build --release || exit $?
clear
target/release/suchain < tests/sample_run.su
