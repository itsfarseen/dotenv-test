# Test to see if we can override `dotenv()`

Contents of `.env`:
```
export FOO=bar
```

Contents of `.env.test`:
```
export FOO=test
```

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/dotenv-test`
FOO=bar


$ source .env.test
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/dotenv-test`
FOO=test
```

# What happens here

`dotenv()` won't overwrite existing env vars.  
This allows us to override env vars by either doing `FOO=xyz cargo run` or `export FOO=xyz; cargo run`.  
In both cases, `dotenv()` won't overwrite `FOO=bar` from the `.env` file.
