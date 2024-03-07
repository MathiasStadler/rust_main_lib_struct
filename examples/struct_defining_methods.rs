// FROM HERE
// https://rust--book-cs-brown-edu.translate.goog/ch05-03-method-syntax.html?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=de

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn width(&mut self) -> u32 {
        self.width
    }

    fn two(&self) -> u32 {
        self.width * self.width
    }
}

impl Rectangle {
    fn square(size: u32) -> u32 {
        size * size
    }
    
    fn set_width2(&mut self, width: u32) -> u32 {
        self.width * width
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    rect1.set_width(30);

    println!("area => {}", rect1.area());

    rect1.set_width(50);

    println!("area => {}", rect1.area());

    println!("{} square pixels.", rect1.two());

    println!("get value =>rect1.with() => {:#?}", rect1.width());

    println!("rect1.set_with() => {:#?}", rect1.set_width2(4));

    println!("square => {:#?}", Rectangle::square(2));
}
