# `cxx`

*A Rust-to-CXX compiler, written in 52 lines of Rust.*

## What is this?

I had an idea that should not work but it works.


## Usage

```
$ cat hello.rs
fn main() {
    println!("Hello, world!");
}
$ rxx -o hello.cpp hello.rs
<prints a command>
$ <copy-past that command>
$ ./hello
Hello, world!
```


## How does it work?

People have a subtle definition of "a valid C++ program". I used that.


## Applicability

None. Never.