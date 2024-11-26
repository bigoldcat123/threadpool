## thread POOL

nothing but for studying rust
```
        let pool = crate::ThreadPool::new(4);
        for i in 0..10 {
            pool.exec(move || {
                println!("hi {:#?}", i);
            })
        }
```