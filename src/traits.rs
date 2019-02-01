pub trait Trait {
    fn display(self);
}

pub trait CTrait: Trait {
    fn hh(self);
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

impl CTrait for i32 {
    fn hh(self) {
        self.display();
        println!("haha");
    }
}

pub fn gi32_dis() -> impl std::fmt::Debug {
    5i32
}

pub fn gi64_dis() -> impl std::fmt::Debug {
    5i64
}

#[cfg(test)]
mod tests {
    // use super::*;
    use super::{gi32_dis, gi64_dis};
    #[test]
    fn test_gi32() {
        for i in 0..7usize {
            println!("{}", i.next_power_of_two())
        }

        println!("{:?} {:?}", gi32_dis(), gi64_dis());
        assert!(true);
    }
}
