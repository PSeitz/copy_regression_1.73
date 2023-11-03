

# Rust 1.73 Performance Regression

Demonstrates a performance regression in Rust 1.73.0.
The program creates a 100Mb Vec and reads it in chunks of 32kb.

Check the flamegraphs for more details where time is spent.

```
➜  copy_regression git:(main) ✗ time cargo run --release
    Finished release [optimized + debuginfo] target(s) in 0.02s
     Running `target/release/copy_regression`

________________________________________________________
Executed in    3.84 secs    fish           external
   usr time    3.80 secs  268.00 micros    3.80 secs
   sys time    0.03 secs  262.00 micros    0.03 secs

➜  copy_regression git:(main) ✗ time cargo +1.72 run --release
    Finished release [optimized + debuginfo] target(s) in 0.03s
     Running `target/release/copy_regression`

________________________________________________________
Executed in   57.23 millis    fish           external
   usr time   16.83 millis  404.00 micros   16.43 millis
   sys time   37.02 millis    0.00 micros   37.02 millis

```
