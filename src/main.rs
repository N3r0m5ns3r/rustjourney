//fn main() {

//    let file = std::fs::read_to_string("lines").unwrap();
//    file
//       .lines()
//        .enumerate()
 //       .filter(|(idx,_)| idx % 2 == 0)
  //      .skip(2)
  //      .take(2)
  //      .for_each(|(_, line)| println!("{}" , line));
//}

enum Color {
    Red,
    Green,
    Blue,

}
impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green 
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue")
    }
}
fn main() {
    print_color(Color::Red);
}
