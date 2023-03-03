fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: 8, y: 5.5 };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

struct Point<Z1, Y1> {
    x: Z1,
    y: Y1,
}

impl<Z1, Y1> Point<Z1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<Z1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
