

# Rust 1.73 Performance Regression

Demonstrates a performance regression in Rust 1.73.0.
The program creates a Vec and reads it in chunks of 32kb. Size can be specified as a command line argument.

Check the flamegraphs for more details where time is spent.

`time cargo run --release 100000000` (If your default is 1.73)
```
Executed in    3.84 secs    fish           external
   usr time    3.80 secs  268.00 micros    3.80 secs
   sys time    0.03 secs  262.00 micros    0.03 secs
```
`time cargo run --release 200000000`
```
Executed in   16.90 secs    fish           external
   usr time   16.82 secs  626.00 micros   16.82 secs
   sys time    0.04 secs    0.00 micros    0.04 secs
```
`time cargo run --release 400000000`
```
Executed in   72.84 secs    fish           external
   usr time   72.65 secs  586.00 micros   72.65 secs
   sys time    0.06 secs  138.00 micros    0.06 secs
```


`time cargo +1.72 run --release  100000000`
```
Executed in   57.23 millis    fish           external
   usr time   16.83 millis  404.00 micros   16.43 millis
   sys time   37.02 millis    0.00 micros   37.02 millis

```

`time cargo +1.72 run --release  400000000`
```
Executed in  257.30 millis    fish           external
   usr time  150.46 millis  477.00 micros  149.98 millis
   sys time  106.64 millis  112.00 micros  106.53 millis
```

