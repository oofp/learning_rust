use serde_json::{Result, Value,};
use core::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Shape2 {
    Circle{r:i32,c:Point},
    Rect{p1:Point,p2:Point},
}


fn main() {
    println!("Hello, json!");
    let _res = untyped_example();
}


fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    point_test();

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn point_test() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    circle_test();
    circle_test2();

    let rect = Shape2::Rect{p1:Point{x:5,y:7},p2:Point{x:15,y:17}}; 
    test_ser(&rect);
}


#[derive(Serialize, Deserialize, Debug)]
struct CircleParams {
    r: i32,
    c: Point
}

#[derive(Serialize, Deserialize, Debug)]
struct RectParams {
    p1: Point,
    p2: Point
}

#[derive(Serialize, Deserialize, Debug)]
enum Shape {
    Circle(CircleParams),
    Rect(RectParams),
}

fn circle_test() {
    let circle:Shape = Shape::Circle(CircleParams{r:10,c:Point { x: 1, y: 2 }});

    // Convert the Point to a JSON string.
    let serialized_circle = serde_json::to_string(&circle).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized_circle = {}", serialized_circle);

    // Convert the JSON string back to a Point.
    let deserialized_circle:Shape = serde_json::from_str(&serialized_circle).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized_circle = {:?}", deserialized_circle);
}

fn circle_test2() {
    let circle:Shape2 = Shape2::Circle{r:10,c:Point { x: 1, y: 2 }};

    // Convert the Point to a JSON string.
    let serialized_circle = serde_json::to_string(&circle).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized_circle2 = {}", serialized_circle);

    // Convert the JSON string back to a Point.
    let deserialized_circle:Shape2 = serde_json::from_str(&serialized_circle).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized_circle2 = {:?}", deserialized_circle);
}

//fn test_ser<T: Deserialize + Serialize + Debug>(t: &T) {
fn test_ser<T: Serialize + Debug>(t: &T) {    
    // Convert the Point to a JSON string .
    let serialized_t = serde_json::to_string(t).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized_t = {}", serialized_t);

    // Convert the JSON string back to a Point.
    //Deserialize +
    //let deserialized_t:T = serde_json::from_str(&serialized_t).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    //println!("deserialized_t = {:?}", deserialized_t);
}
