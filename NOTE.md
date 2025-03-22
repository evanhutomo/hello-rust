mod
- if you declare mod aaa, rust will looks for a file name aaa.rs or a directory named aaa with mod.rs
- we can explicitly set the path of the mod like below

#[path = "geometry/mod123.rs"]
mod geometry