/**
 * Simple use of Generics in Structs
 */

struct Point<T> {
    x: T,
    y: T,
}

struct Area<T, U> {
    id: T,
    area: U,
}

fn main() {
    let some_point = Point { x: 10, y: 21 };
    // let some_point: Point<u32> = Point { x: 10, y: 21 }; //It works as well
    println!("Point x: {} y: {}", some_point.x, some_point.y);

    let some_area = Area { id: 10, area: 10.5 };
    println!("Area id: {} area: {}", some_area.id, some_area.area);
}
