#!/bin/bash

cargo build --release
cd target/release
strip python_core_rust

echo 'Resulting file:'
ls -la python_core_rust

