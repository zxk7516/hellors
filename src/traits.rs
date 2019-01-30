pub trait Trait {
    fn display(self);
}

impl Trait for i32 {
    fn display(self) {
        println!("{}", self);
    }
}

impl Trait for i64 {
    fn display(self) {
        println!("{}", self);
    }
}

pub fn gi32() -> impl Trait {
    5i32
}

pub fn gi64() -> impl Trait {
    5i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gi32() {
        gi32().display();
        gi64().display();
        assert!(true);
    }
}
