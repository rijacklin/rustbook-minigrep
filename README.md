# Minigrep (Rust Book Project)

This repo contains an implementation of the project tutorial in Chapter 12 of
the book, [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

The application is called `minigrep`, a simplified implementation of the `grep` utility (*g*lobally search for a *re*gular expression and *p*rint).

Implementation follows that tutorial closely. It diverges in one situation by
using lifetimes and string slices (`&str`) for the `Config` structure rather
than following the tutorial's usage of the `.clone()` method. This is done as
an improvement and should technically result in better performance. However,
that performance gain is likely negligible due to the limited scope of the
application.
