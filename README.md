# Example of Scala to Rust binding via JNI

## Build and run
```
$ sbt javah && cargo build --release --manifest-path=rusting/Cargo.toml && sbt run
```