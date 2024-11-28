fn main() {
  println!("**1. ¿Qué es un enum en Rust?**");
  println!("Un enum (enumeración) es un tipo de dato definido por el usuario que permite enumerar valores posibles, llamados variantes.");
  println!("Se usa para representar opciones o estados, y facilita el uso de patrones `match` para manejar cada variante.\n");

  // **2. Enum básico**
  println!("**2. Ejemplo básico de un enum:**");
  enum TrafficLight {
      Red,
      Yellow,
      Green,
  }

  // Usamos el operador `::` para acceder a las variantes del enum
  let light = TrafficLight::Red;
  match light {
      TrafficLight::Red => println!("¡Detente! La luz es roja."),
      TrafficLight::Yellow => println!("¡Precaución! La luz es amarilla."),
      TrafficLight::Green => println!("¡Avanza! La luz es verde."),
  }

  // **3. Enum con datos asociados**
  println!("\n**3. Enum con datos asociados:**");
  enum TrafficLightWithTime {
      Red(u8), // La luz roja dura un número de segundos
      Yellow(char), // La luz amarilla tiene un símbolo asociado
      Green(String), // La luz verde tiene un mensaje asociado
  }

  let red_light = TrafficLightWithTime::Red(10);
  let yellow_light = TrafficLightWithTime::Yellow('Y');
  let green_light = TrafficLightWithTime::Green(String::from("Avanza con cuidado."));

  // Usamos match para manejar cada variante
  match red_light {
      TrafficLightWithTime::Red(seconds) => println!("La luz roja dura {} segundos.", seconds),
      TrafficLightWithTime::Yellow(symbol) => println!("La luz amarilla tiene el símbolo '{}'.", symbol),
      TrafficLightWithTime::Green(message) => println!("Mensaje de la luz verde: {}", message),
  }

  // **4. Uso del enum Option**
  println!("\n**4. Uso del enum Option para manejar valores opcionales:**");

  fn divide(x: f64, y: f64) -> Option<f64> {
      if y == 0.0 {
          None // Devuelve None si hay división por cero
      } else {
          Some(x / y) // Devuelve el resultado envuelto en Some
      }
  }

  let result = divide(10.0, 2.0);
  match result {
      Some(value) => println!("El resultado de la división es: {}", value),
      None => println!("Error: División por cero."),
  }

  let error_result = divide(10.0, 0.0);
  match error_result {
      Some(value) => println!("El resultado de la división es: {}", value),
      None => println!("Error: División por cero."),
  }

  // **5. Resumen**
  println!("\n**5. Resumen:**");
  println!("- Los enums permiten definir variantes de un tipo de dato.");
  println!("- Pueden tener datos asociados para representar información adicional.");
  println!("- El patrón `match` se utiliza para manejar las variantes.");
  println!("- El enum `Option` es útil para manejar valores opcionales y evitar errores de referencia nula.");


  println!("**Solución: Enums en Rust**");

  // **1. Enum básico**
  enum Weather {
      Sunny,
      Rainy,
      Cloudy,
  }

  let today = Weather::Sunny;
  match today {
      Weather::Sunny => println!("El clima está soleado. ¡Disfruta el día!"),
      Weather::Rainy => println!("Está lloviendo. Lleva un paraguas."),
      Weather::Cloudy => println!("El clima está nublado. Tal vez llueva más tarde."),
  }

  // **2. Enum con datos asociados**
  enum Shape {
      Circle(f64),           // Radio del círculo
      Rectangle(f64, f64),   // Largo y ancho del rectángulo
  }

  let circle = Shape::Circle(3.0);
  let rectangle = Shape::Rectangle(4.0, 5.0);

  match circle {
      Shape::Circle(radius) => println!("Área del círculo: {:.2}", std::f64::consts::PI * radius * radius),
      Shape::Rectangle(_, _) => {} // No hacemos nada aquí
  }

  match rectangle {
      Shape::Circle(_) => {} // No hacemos nada aquí
      Shape::Rectangle(length, width) => println!("Área del rectángulo: {:.2}", length * width),
  }

  // **3. Uso del enum Option**
  fn find_in_array(arr: &[i32], target: i32) -> Option<usize> {
      for (index, &value) in arr.iter().enumerate() {
          if value == target {
              return Some(index);
          }
      }
      None
  }

  let numbers = [1, 2, 3, 4, 5];
  let target = 3;

  match find_in_array(&numbers, target) {
      Some(index) => println!("El valor {} se encuentra en la posición {}.", target, index),
      None => println!("El valor {} no está en el arreglo.", target),
  }

  let missing_target = 10;
  match find_in_array(&numbers, missing_target) {
      Some(index) => println!("El valor {} se encuentra en la posición {}.", missing_target, index),
      None => println!("El valor {} no está en el arreglo.", missing_target),
  }

  println!("¡Ejercicio completado correctamente!");
}
