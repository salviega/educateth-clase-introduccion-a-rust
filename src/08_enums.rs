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



  println!("**Ejercicio práctico: Enums en Rust**");

  // **1. Enum básico**
  println!("1. Declara un enum llamado `Weather` con las variantes `Sunny`, `Rainy`, y `Cloudy`.");
  println!("   Usa `match` para imprimir un mensaje específico basado en el valor del enum.\n");

  // **2. Enum con datos asociados**
  println!("2. Define un enum llamado `Shape` con las variantes `Circle(f64)` y `Rectangle(f64, f64)`.");
  println!("   Usa `match` para calcular e imprimir el área de cada variante.\n");

  // **3. Uso del enum Option**
  println!("3. Escribe una función llamada `find_in_array` que reciba un arreglo de enteros y un valor a buscar.");
  println!("   Retorna un `Option<usize>` indicando la posición del valor si se encuentra o `None` si no está.");
  println!("   Usa esta función y `match` para manejar ambos casos.\n");

  println!("¡Resuelve los ejercicios para aprender más sobre Enums en Rust!");
}
