
enum Color {
  Red,
  Green,
  Blue
}

impl Color {
  fn print(&self) {
    match self {
      Self::Red => println!("Color: Red"),
      Self::Green => println!("Color: Green"),
      Self::Blue => println!("Color: Blue")
    }
  }
}

struct Dimensions {
  height: f32,
  width: f32,
  length: f32
}

impl Dimensions {

  fn print(&self) {
    println!("Height: {}", self.height);
    println!("Width: {}", self.width);
    println!("Length: {}", self.length);
  }
  
}


struct ShippingBox {
  dimensions: Dimensions,
  weight: i32,
  color: Color
  
}

impl ShippingBox {

  fn new(dimension: Dimensions, 
         weight: i32, color: Color) -> Self {
    Self { 
      dimensions: dimension,
      weight: weight, 
      color: color }
  }

  fn get(&self) {

    self.color.print();
    self.dimensions.print();
    println!("Weight: {}", self.weight)
    
  }
}


fn main() {

  let new_dimensions = Dimensions {

    height:2.0, 
    width:3.0, 
    length:5.0
    
  };
    
  let new_box: ShippingBox = ShippingBox::new(
               new_dimensions,
                4,
                Color::Red
                
          
  );

  new_box.get();
  
}