#!/bin/bash

# Build the CLI

echo "Debug or Release build? (d/r)"
read option

if [ $option = "d" ]; then
    echo "Building Debug..."
    cargo build --bin mono-cli
else
    echo "Building Release..."
    cargo build --release --bin mono-cli
fi