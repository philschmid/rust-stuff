# How to work with Tokio & Asyncio ehm i mean Rust and Python 


## Getting started

```bash
cargo run
```
send request
```Bash
curl --request POST \
  --url http://localhost:3000/predict \
  --header 'Content-Type: application/json' \
  --data '{"inputs":"I like you"}'
```

## Run Test

```bash
cargo run --release
```

send load

```Bash
hey -n 1000 -m POST -H 'Content-Type: application/json' -d '{	"inputs": "I love you. I like you. I am your friend."}' http://127.0.0.1:3000/predict
```
**Result CPU:**

```bash
Summary:
  Total:        30.9108 secs
  Slowest:      1.5758 secs
  Fastest:      0.1874 secs
  Average:      1.5374 secs
  Requests/sec: 32.3512
  
  Total data:   40000 bytes
  Size/request: 40 bytes

Response time histogram:
  0.187 [1]     |
  0.326 [5]     |
  0.465 [0]     |
  0.604 [0]     |
  0.743 [0]     |
  0.882 [0]     |
  1.020 [0]     |
  1.159 [0]     |
  1.298 [0]     |
  1.437 [0]     |
  1.576 [994]   |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
```


**Result GPU:**

```bash

Summary:
  Total:        4.1857 secs
  Slowest:      0.2179 secs
  Fastest:      0.0464 secs
  Average:      0.2075 secs
  Requests/sec: 238.9092
  
  Total data:   40000 bytes
  Size/request: 40 bytes

Response time histogram:
  0.046 [1]     |
  0.064 [9]     |
  0.081 [0]     |
  0.098 [0]     |
  0.115 [0]     |
  0.132 [0]     |
  0.149 [0]     |
  0.166 [0]     |
  0.184 [0]     |
  0.201 [0]     |
  0.218 [990]   |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
```

## Resources 

* [My Github Issue](https://github.com/awestlake87/pyo3-asyncio/issues/60)
* [Documentation: Asyncio Event Loop](https://awestlake87.github.io/pyo3-asyncio/master/doc/pyo3_asyncio/#event-loop-references-and-contextvars)
* [Robyn execution in python](https://github.com/sansyrox/robyn/blob/main/src/processor.rs) 
