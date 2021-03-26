#!/bin/bash

cd /code
rm -fr sparql.db
cargo build --release
./target/release/create-db
cp -r sparql.db ./target/release
cd target/release/
rm -f lambda.zip
zip -r lambda.zip bootstrap sparql.db/
mv lambda.zip ../../
rm -fr sparql.db
cd ../../
