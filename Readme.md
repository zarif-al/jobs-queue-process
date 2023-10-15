# Sanity Custom Sync

This is a Rust implementation of an API server that will queue and execute jobs.

The goal of this application is to create a multithreaded solution to queue jobs sent from `Sanity Connect` in shopify and execute them.

### Benchmark

There is another [project](https://github.com/lemon-hive/sanity-custom-sync) that implements a similar solution using NestJS. We will consider that as the benchmark.

