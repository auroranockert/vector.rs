#[link(name = "vector", vers = "0.1", uuid = "bd080482-e281-48ee-bb19-917f6b4a466f")];

#[license = "MIT"];
#[crate_type = "lib"];

#[author = "Jens Nockert"];

#[comment = "Simplify working with vectors in Rust"];
#[desc = "Additional methods and functions that operate on vectors"];

pub trait Iterators<T> {
    fn each_cons(&self, n: uint, iterator: &fn(&[T]) -> bool) -> bool;
    fn to_cons(&self, n: uint) -> ~[~[T]];
}

impl<T: Copy> Iterators<T> for ~[T] {
    fn each_cons(&self, n: uint, iterator: &fn(&[T]) -> bool) -> bool {
        uint::iterate(0, self.len() / n, |i| {
            let start = i * n;
            iterator(self.slice(start, start + n))
        })
    }

    fn to_cons(&self, n: uint) -> ~[~[T]] {
        let mut result = ~[];
        for self.each_cons(n) |v| { result.push(v.to_owned()) }
        result
    }
}
