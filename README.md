# Learning Rust

## Continue/ Tasks to do after course done
- [ ] extend the request and response implementation. Eg handle HTTP Headers
- [ ] increase performance. atm we run single thread. look at std::thread and std::sync to explore mutlithreading 
- [ ] when familiar with multithread we can stard with async --> tokio.rs + async await


# Notes
## gdb Rust u16 Bug
weird pwndbg bug (https://github.com/pwndbg/pwndbg/issues/855)
just run following line at the beginning:
```
pwndbg> set language c
```

## Source
Based on udemy course 'Learn Rust by Building Real Applications'
![Alt text](UC-65f03263-b59f-4406-a03e-3987b9a14ac8.jpg?raw=true "tada")
