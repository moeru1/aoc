#!/usr/bin/env bash
#
cargo new $1
rm ./$1/src -r
cp ./template/src ./$1/src -r
cd $1

