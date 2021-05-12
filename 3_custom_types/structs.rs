#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let (Point {x: left, y: top}, 
        Point {x: right, y: bottom}) 
        = (rect.top_left, rect.bottom_right);
    let area: f32 = (right - left) * (top - bottom);
    return area;
}

fn square(point: Point, side_len: f32) -> Rectangle {
    println!("left: {:?}, bottom: {:?}", point.x, point.y);


    return Rectangle {
        top_left: Point {x: point.x, y: point.y + side_len }, 
        bottom_right: Point {x: point.x + side_len, y: point.y }
    };
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`

    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { y: left_edge, x: top_edge } = point;
    println!("Point: {:?}, top_edge: {:?}, left_edge {:?}", point, top_edge, left_edge);
    

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: 2.0, y: 5.0 },
        bottom_right: Point { x: 12.0, y: 1.0},
    };
    println!("{}", rect_area(rectangle));

    // creates a square from a given point
    let bottom_left = Point {x: 1.0, y: 1.0};
    println!("{:?}", square(bottom_left, 3.0));


    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
