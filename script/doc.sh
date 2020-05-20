#! /usr/bin/env bash

# generate docs with mathjax
RUSTDOCFLAGS="--html-in-header script/doc.header.html" cargo doc --no-deps
