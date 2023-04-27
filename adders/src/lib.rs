#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, r1: &Rectangle) -> bool {
        self.width > r1.width && self.height > r1.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let small = Rectangle {
            width: 5,
            height: 1,
        };
        let large = Rectangle {
            width: 7,
            height: 8,
        };

        assert!(large.can_hold(&small))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let small = Rectangle {
            width: 5,
            height: 1,
        };
        let large = Rectangle {
            width: 7,
            height: 8,
        };

        assert!(!small.can_hold(&large))
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("fails")
    }
}
