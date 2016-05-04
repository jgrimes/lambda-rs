# lambda-rs
lambda calculus interpreters in Rust

This is the first thing I've written in Rust, so I am sure it is full of problems -- let me know if you notice any! 
Many methods need to be reworked to do the right (in terms of efficiency) things with regards to borrowing/cloning/etc.

Currently, only the untyped lambda calculus interpreter is done, based on [this](http://augustss.blogspot.com/2007/10/simpler-easier-in-recent-paper-simply.html) blog post by Lennart Augustsson
