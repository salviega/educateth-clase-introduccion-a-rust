fn main() {
  println!("**1. ¿Qué son los Genéricos en Rust?**");
  println!("Los genéricos permiten escribir código reutilizable y flexible sin especificar un tipo fijo.");
  println!("Usan parámetros como `<T>`, `<K, V>`, etc., que representan tipos genéricos.");
  println!("Ejemplos comunes en Rust son estructuras como Vec<T> o HashMap<K, V>.\n");

  // **2. Ejemplo básico sin genéricos**
  println!("**2. Ejemplo básico sin genéricos:**");
  println!("¿Qué pasa si necesitamos encontrar el elemento más grande en listas de diferentes tipos?");
  println!("Primero, implementamos dos funciones separadas para i32 e i64.\n");

  fn largest_for_i32(list: &[i32]) -> i32 {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  fn largest_for_i64(list: &[i64]) -> i64 {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  let nums_i32 = [1, 2, 3, 4, 5];
  let nums_i64 = [10, 20, 30, 40, 50];

  println!("Mayor en i32: {}", largest_for_i32(&nums_i32));
  println!("Mayor en i64: {}", largest_for_i64(&nums_i64));

  // **3. Usando Genéricos**
  println!("\n**3. Usando Genéricos:**");
  println!("Ahora usamos genéricos para resolver el mismo problema de manera elegante.");
  println!("Con genéricos, evitamos duplicar lógica y soportamos múltiples tipos.\n");

  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  let nums_f64 = [3.5, 7.2, 1.8, 9.4];
  println!("Mayor en f64: {}", largest(&nums_f64));

  // **4. Genéricos en estructuras**
  println!("\n**4. Genéricos en estructuras:**");
  println!("Los genéricos también pueden usarse en estructuras para definir miembros de diferentes tipos.");
  struct Point1<T> {
      x: T,
      y: T,
  }

  struct Point2<T, U> {
      x: T,
      y: U,
  }

  let int_point = Point1 { x: 5, y: 10 };
  let float_point = Point1 { x: 1.0, y: 4.0 };
  let mixed_point = Point2 { x: 5, y: 3.2 };

  println!("Punto int: ({}, {})", int_point.x, int_point.y);
  println!("Punto float: ({}, {})", float_point.x, float_point.y);
  println!("Punto mixto: ({}, {})", mixed_point.x, mixed_point.y);

  // **5. Genéricos en enums**
  println!("\n**5. Genéricos en enums:**");
  println!("Los enums genéricos, como Option<T>, permiten trabajar con cualquier tipo.");
  enum Option<T> {
      Some(T),
      None,
  }

  let int_option = Option::Some(42);
  let float_option = Option::Some(4.2);
  println!("Option int y float creados.");

  // **6. Genéricos en métodos**
  println!("\n**6. Genéricos en métodos:**");
  println!("Podemos definir métodos genéricos para estructuras con genéricos.");
  impl<T> Point1<T> {
      fn get_x(&self) -> &T {
          &self.x
      }
  }

  let x_value = int_point.get_x();
  println!("El valor de x en el punto int es: {}", x_value);

  // **7. Resumen**
  println!("\n**Resumen:**");
  println!("- Los genéricos hacen el código más flexible y reutilizable.");
  println!("- Se usan en estructuras, enums y métodos.");
  println!("- Al escribir funciones genéricas, recuerda agregar restricciones como `PartialOrd` o `Copy` cuando sea necesario.");
  println!("- Esto mejora la mantenibilidad y reduce duplicación de código.");


  println!("**Ejercicio práctico: Genéricos en Rust**");

  // **1. Función genérica**
  println!("1. Escribe una función genérica llamada `menor` que reciba un slice de elementos y retorne el menor.");
  println!("   Asegúrate de usar restricciones como `PartialOrd` y `Copy`.\n");

  // **2. Estructuras genéricas**
  println!("2. Define una estructura genérica llamada `Pair<T, U>` con dos campos de tipos diferentes.");
  println!("   Crea un par con valores de diferentes tipos y accede a ellos.\n");

  // **3. Métodos genéricos**
  println!("3. Añade un método genérico a la estructura `Pair` que retorne una tupla con ambos valores.\n");

  // **4. Enums genéricos**
  println!("4. Define un enum genérico llamado `Result<T, E>` con dos variantes: `Ok(T)` y `Err(E)`.");
  println!("   Crea una instancia de cada variante y usa `match` para manejarlas.\n");

  println!("¡Completa estos ejercicios para practicar genéricos en Rust!");

}
