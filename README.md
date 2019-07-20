# Example of Scala to Rust binding via JNI

## Build

### Generate header files
```
$ sbt javah
```

### Build rust library
```
$ cargo build --release
```

### Run
```
$ sbt run
```