#!/bin/bash

echo "md5 algorithm node vs rust"

cd rust
cargo run

cd ..

cd node 
npm start

cd ..