#[derive(Debug)]


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /*
    fn somme(&self) -> u32 {
        self.width * self.height
    }
    */
}


fn main() {
    //let width1: u32 = 22;
    //let height1: u32 = 22;
    // let rect: (u32, u32) = (22, 22);

    let scale: u32 = 4;
    let rect_un: Rectangle = Rectangle {width: 22, height: 22};
    let rect_deux: Rectangle = Rectangle {
        width: dbg!(50 * scale),
        height: 131
    };
    
    // println!("The area of the rectangle is {} square pixels.", area(rect));
    println!("The area of the rectangle is {} square pixels.", area(&rect_un));

    // println!("rect_deux is {:?}", rect_deux);
    // println!("rect_deux is {:#?}", rect_deux);
    dbg!(&rect_deux);

    let rect_trois: Rectangle = Rectangle{
        width: 64, height: 29
    };

    println!("rect_trois is {}", rect_trois.area());
}


fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
