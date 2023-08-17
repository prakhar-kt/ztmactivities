
///  A Student struct
struct Student {

  name: String,
  locker: Option<i32>
  
}

/// Implemetations for the Student struct
impl Student {
  fn new(name: String, locker: Option<i32>) -> Self {

    Self { name: name, locker: locker }
    
  }
  /// prints the Student details
  fn print(&self) {
    
    match self.locker {
      Some(num) => {
        println!("{} has been assigned locker number {:?}", 
                 self.name, num)
      },
      None => {
        println!("{} has not been assigned any locker", 
                self.name)
      }
    }
  }
}

fn main() {

  let student1_name = "John".to_string();
  let student2_name = "Mary".to_string();
  

  let students = vec![
    Student::new(student1_name, Some(32)),
    Student::new(student2_name, None)
  ];

  for student in students {
    student.print();
  }
  
}