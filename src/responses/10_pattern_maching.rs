fn main() {
  println!("**1. ¿Qué es Pattern Matching en Rust?**");
  println!("Pattern Matching permite comparar un valor objetivo con una serie de patrones.");
  println!("Rust utiliza `match` y `if let` para trabajar con patrones de manera segura y expresiva.");
  println!("`match` debe ser exhaustivo, manejando todas las posibilidades de un patrón, y para los casos no listados, usamos `_`.\n");

  // **2. Ejemplo básico con enum**
  println!("**2. Ejemplo básico con enum:**");
  enum BlockChain {
      BitCoin,
      Ethereum,
      Starknet,
      Solana,
  }

  let block_chain = BlockChain::Solana;
  match block_chain {
      BlockChain::BitCoin => println!("BitCoin"),
      BlockChain::Ethereum | BlockChain::Starknet => println!("Ethereum o Starknet"),
      _ => println!("Solana"), // Maneja todas las posibilidades restantes
  }

  // **3. Destructuración con enum**
  println!("\n**3. Destructuración con enum:**");
  enum Shape {
      Circle(f64),
      Rectangle(f64, f64),
      Square(f64),
  }

  fn calculate_area(shape: &Shape) -> f64 {
      match shape {
          Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
          Shape::Rectangle(width, height) => width * height,
          Shape::Square(side) => side * side,
      }
  }

  let circle = Shape::Circle(3.0);
  let rectangle = Shape::Rectangle(4.0, 5.0);
  let square = Shape::Square(2.0);

  println!("Área del círculo: {}", calculate_area(&circle));
  println!("Área del rectángulo: {}", calculate_area(&rectangle));
  println!("Área del cuadrado: {}", calculate_area(&square));

  // **4. Destructuración de estructuras**
  println!("\n**4. Destructuración de estructuras:**");
  struct Point {
      x: i32,
      y: i32,
  }

  fn process_point(point: Point) {
      match point {
          Point { x: 0, y: 0 } => println!("El punto está en el origen."),
          Point { x, y } => println!("El punto está en ({}, {}).", x, y),
      }
  }

  let point1 = Point { x: 0, y: 0 };
  let point2 = Point { x: 3, y: 7 };
  process_point(point1);
  process_point(point2);

  // **5. Uso de `if let` para simplificar patrones**
  println!("\n**5. Uso de `if let`:**");
  let some_u8_value = Some(3u8);

  match some_u8_value {
      Some(3) => println!("Es un tres (con match)."),
      _ => (),
  }

  // Simplificación con `if let`
  if let Some(3) = some_u8_value {
      println!("Es un tres (con if let).");
  }

  // **6. Uso de `match` como expresión**
  println!("\n**6. Uso de `match` como expresión:**");
  let shape = Shape::Circle(5.0);
  let area = match shape {
      Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
      Shape::Rectangle(width, height) => width * height,
      Shape::Square(side) => side * side,
  };
  println!("Área calculada: {}", area);

  println!("\n**Resumen:**");
  println!("- `match` permite manejar todas las posibilidades de un patrón.");
  println!("- Los enums y estructuras son fáciles de destructurar con `match`.");
  println!("- `if let` simplifica el manejo de un caso específico.");
  println!("- `match` también puede actuar como expresión, retornando valores útiles.");


  println!("**Solución: Pattern Matching en Rust**");

  // **1. Uso básico de `match`**
  enum Weather {
      Sunny,
      Rainy,
      Cloudy,
  }

  let today = Weather::Rainy;
  match today {
      Weather::Sunny => println!("El clima está soleado. ¡Disfruta tu día!"),
      Weather::Rainy => println!("Está lloviendo. Lleva un paraguas."),
      Weather::Cloudy => println!("El clima está nublado. Quizás llueva más tarde."),
  }

  // **2. Destructuración con enum**
  enum Shape {
      Circle(f64),           // Radio del círculo
      Rectangle(f64, f64),   // Largo y ancho del rectángulo
      Square(f64),           // Lado del cuadrado
  }

  fn calculate_area(shape: &Shape) -> f64 {
      match shape {
          Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
          Shape::Rectangle(width, height) => width * height,
          Shape::Square(side) => side * side,
      }
  }

  let circle = Shape::Circle(3.0);
  let rectangle = Shape::Rectangle(4.0, 5.0);
  let square = Shape::Square(2.0);

  println!("Área del círculo: {}", calculate_area(&circle));
  println!("Área del rectángulo: {}", calculate_area(&rectangle));
  println!("Área del cuadrado: {}", calculate_area(&square));

  // **3. Destructuración de estructuras**
  struct Person {
      name: String,
      age: u8,
  }

  let person1 = Person {
      name: String::from("Juan"),
      age: 16,
  };
  let person2 = Person {
      name: String::from("Ana"),
      age: 65,
  };

  fn describe_person(person: Person) {
      match person {
          Person { age, .. } if age < 18 => println!("Es menor de edad."),
          Person { age, .. } if age > 60 => println!("Es una persona mayor."),
          Person { name, age } => println!("{} tiene {} años y está en su mejor momento.", name, age),
      }
  }

  describe_person(person1);
  describe_person(person2);

  // **4. Uso de `if let`**
  let some_number = Some(42);

  if let Some(42) = some_number {
      println!("El número es 42.");
  } else {
      println!("El número no es 42.");
  }

  println!("¡Ejercicio completado correctamente!");
}
