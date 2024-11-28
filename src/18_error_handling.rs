fn main() {
  println!("**1. ¿Qué es el manejo de errores en Rust?**");
  println!("Rust divide los errores en dos categorías principales:");
  println!("1. Errores irrecuperables (`panic!`): Situaciones críticas donde el programa no puede continuar.");
  println!("2. Errores recuperables (`Result<T, E>`): Situaciones que el programa puede manejar y continuar ejecutándose.\n");

  // **2. Errores irrecuperables: panic!**
  println!("**2. Errores irrecuperables: `panic!`**");
  println!("El macro `panic!` se usa para situaciones donde el programa no puede recuperarse.");
  println!("Cuando ocurre un `panic!`, Rust limpia la pila (stack unwinding) y termina el programa.\n");

  // Ejemplo de panic
  // Descomenta esta línea para probar un error irrecuperable
  // panic!("Este es un error irrecuperable, el programa se detendrá.");

  println!("Nota: Evita usar `panic!` excepto para errores críticos que no se pueden manejar.\n");

  // **3. Errores recuperables: Result<T, E>**
  println!("**3. Errores recuperables: `Result<T, E>`**");
  println!("El tipo `Result` se utiliza para manejar errores recuperables de forma segura.");
  println!("`Result` tiene dos variantes:");
  println!("- `Ok(T)`: Indica éxito, contiene el valor retornado.");
  println!("- `Err(E)`: Indica un error, contiene información sobre el error.\n");

  // Ejemplo básico con Result
  fn divide(a: i32, b: i32) -> Result<i32, String> {
      if b == 0 {
          Err(String::from("No se puede dividir entre cero"))
      } else {
          Ok(a / b)
      }
  }

  match divide(10, 2) {
      Ok(result) => println!("Resultado: {}", result),
      Err(err) => println!("Error: {}", err),
  }

  match divide(10, 0) {
      Ok(result) => println!("Resultado: {}", result),
      Err(err) => println!("Error: {}", err),
  }

  println!("\n**4. Propagación de errores con `?`**");
  println!("El operador `?` simplifica la propagación de errores, evitando código anidado.");
  println!("`?` retorna el valor `Ok` si la operación es exitosa, o propaga el `Err` al llamador.\n");

  use std::fs::File;
  use std::io::{self, Read};

  fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
      let mut file = File::open(file_path)?; // Intenta abrir el archivo
      let mut contents = String::new();
      file.read_to_string(&mut contents)?; // Lee el contenido
      Ok(contents) // Retorna el contenido si es exitoso
  }

  match read_file_contents("example.txt") {
      Ok(contents) => println!("Contenido del archivo:\n{}", contents),
      Err(err) => eprintln!("Error leyendo el archivo: {}", err),
  }

  println!("\n**5. Resumen:**");
  println!("- Usa `panic!` solo para errores críticos que no se pueden manejar.");
  println!("- Usa `Result<T, E>` para manejar errores recuperables.");
  println!("- El operador `?` simplifica la propagación de errores.");
  println!("- Rust garantiza la seguridad y claridad en el manejo de errores.\n");

  println!("**Ejercicio 1: Errores Irrecuperables**");
  println!("1. Usa el macro `panic!` para simular un error crítico en el programa.");
  println!("   Observa cómo Rust detiene la ejecución y muestra el mensaje de error.\n");

  println!("**Ejercicio 2: Manejo de Errores Recuperables con `Result`**");
  println!("2. Escribe una función llamada `divide` que reciba dos enteros y retorne un `Result<i32, String>`.");
  println!("   - Si el divisor es cero, la función debe retornar un `Err` con un mensaje apropiado.");
  println!("   - Si no, debe retornar el resultado de la división dentro de un `Ok`.\n");

  println!("**Ejercicio 3: Propagación de Errores con `?`**");
  println!("3. Escribe una función llamada `leer_archivo` que reciba un nombre de archivo como parámetro.");
  println!("   Usa el operador `?` para manejar errores al abrir y leer el archivo.");
  println!("   Retorna el contenido del archivo como un `Result<String, io::Error>`.\n");

  println!("¡Completa estos ejercicios para practicar el manejo de errores en Rust!");
}
