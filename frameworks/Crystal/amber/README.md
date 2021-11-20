# Welcome to Amber

This is the [Amber](https://amberframework.org) test of the Framework Benchmarks.

[![Amber logo](https://i.imgur.com/NEodgHV.png)](https://amberframework.org)

**Amber** is a web application framework written in [Crystal](http://www.crystal-lang.org) inspired by Kemal, Rails, Phoenix and other popular application frameworks.

* [User Guide](https://docs.amberframework.org/amber)
* [API Documentation](https://docs.rs/axum/0.3.0/axum/)
* Cargo package: [axum](https://crates.io/crates/axum)

## Database

PostgreSQL

* Orm using [granite](https://github.com/amberframework/granite)

## Test URLs

### Test 1: JSON Encoding

    http://localhost:8080/json

### Test 2: Single Row Query

    http://localhost:8080/db

### Test 3: Multi Row Query

    http://localhost:8080/queries?queries=20

### Test 4: Fortunes (Template rendering)

    http://localhost:8080/fortunes

### Test 5: Update Query

    http://localhost:8080/updates?queries=20

### Test 6: Plaintext

    http://localhost:8080/plaintext

