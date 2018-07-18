# Curate Issue

[![Build Status](https://travis-ci.com/rust-community/curate-issue.svg?branch=master)](https://travis-ci.com/rust-community/curate-issue)

## Running the tool
`cargo run path/to/issue path/to/xml_file`

## Example
`cargo run rust-community/content-o-tron/issues/6 rss_feed.xml`

*If you specify a rss file that already exists the items will be appended.*

## Documentation

You can generate the source code documentation by running `cargo rustdoc -- --document-private-items`
