# rustracer

A simple ray-tracing render engine written in Rust, based on the book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).
The book was a joy to follow along and I highly recommend having a look at it :)

![Sample Output](samples/output-540p.png)


## Running This Project

Simply execute `cargo run --release` in project root to render the book cover scene.
The release flag is needed as rendering without optimizations is painfully slow.
