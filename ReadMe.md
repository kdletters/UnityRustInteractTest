This project is designed to attempt the interaction between Rust and Unity(C#).

Because of the difference between Rust and C#, string and array(Vec in Rust and List or Array in C#) can not directly pass to each other, so there's a buffer([ByteBuffer](https://github.com/Cysharp/csbindgen/?tab=readme-ov-file#string-and-arrayspan)) in project to resolve this problem.

# TODO

- Gather all dlls in a directory