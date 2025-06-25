pub mod union_find;

pub mod prelude {
    pub use crate::union_find::*;
}

#[cfg(test)]
mod tests {
    use crate::union_find::*;

    #[test]
    fn construction() {
        UnionFind::new(10);
    }
}
