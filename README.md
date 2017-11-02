#q m-method-rs
===================
----------

This is my implementation of the Quine-McCluskey Method in rust. It can currently support up to 52 terms and 2^52 m terms (obviously). It's a moderately inefficient and has only been tested with 4 terms

## TO USE
It expects an integer from 2 to 52 for the number of terms. It should handle invalid input so....

It then expects a list of terms with comas seperating like sooo "1 ,4,5  ,7" As shown spaces don't matter
(they are trimmed off)

![alt text](https://github.com/OneMoreByte/qm-method-rs/blob/master/expected-input.png "An example run")


## TO RUN

Get the compiler here -> [https://www.rust-lang.org/en-US/install.html] (https://www.rust-lang.org/en-US/install.html)

Then go into the folder and run "cargo build --target=x86_64-pc-windows-msvc" (assuming you're on windows)
just "cargo build" might work tooo

it should output the binary in ./target/x86_64-pc-windows-msvc/debug/ or just /target/debug if you just ran "cargo build" and it worked
