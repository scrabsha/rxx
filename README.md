# `cxx`

*A Rust-to-CXX compiler, written in 65 lines of Rust.*

## What is this?

I had an idea that should not work but it works.


## Usage

```
$ cat hello.rs
fn main() {
    println!("Hello, world!");
}
$ rxx -o hello.cpp hello.rs
gcc -o main main.cxx /home/ssha/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-*.so
$ gcc -o main main.cxx /home/ssha/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-*.so
$ ./hello
Hello, world!
```


## How does it work?

People have a subtle definition of "a valid C++ program". I used that.


## Applicability

None. Never.