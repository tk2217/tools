#!/bin/sh

# TODO: Use something better to run the build.

mkdir -p public
rm -rf public/*
cp -r output/* public
cp -r static/* public
