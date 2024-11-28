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


  println!("**Ejercicio práctico: Pattern Matching en Rust**");

  // **1. Uso básico de `match`**
  println!("1. Declara un enum llamado `Weather` con las variantes `Sunny`, `Rainy`, y `Cloudy`.");
  println!("   Usa `match` para imprimir un mensaje dependiendo del clima actual.\n");

  // **2. Destructuración con enum**
  println!("2. Define un enum llamado `Shape` con las variantes `Circle(f64)`, `Rectangle(f64, f64)` y `Square(f64)`.");
  println!("   Escribe una función que calcule el área de cada forma usando `match`.\n");

  // **3. Destructuración de estructuras**
  println!("3. Declara una estructura llamada `Person` con los campos `name` y `age`.");
  println!("   Usa `match` para imprimir un mensaje personalizado si la persona tiene menos de 18 años, más de 60 años o si está en un rango intermedio.\n");

  // **4. Uso de `if let`**
  println!("4. Declara una opción (enum `Option`) que contenga un valor entero.");
  println!("   Usa `if let` para manejar el caso en que el valor sea igual a 42.\n");

  println!("¡Resuelve estos ejercicios para dominar Pattern Matching en Rust!");

}
