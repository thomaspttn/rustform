# RustForm
![License](https://img.shields.io/badge/license-MIT-blue.svg)

Example code to turn Google Form results into an HTML page using Rust!

![alt text](example.png)

Parsing through the responses of a Google Form is annoying through their default interface and viewing the results in Google Sheets isn't much better. So I wrote this sample Rust program which takes an input `data.csv` and uses the [askama](https://crates.io/crates/askama) crate to populate a predefined HTML template.

This probably would have been easier in Python, but I was looking for an excuse to use Rust!!


