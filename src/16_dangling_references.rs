fn main() {
  println!("**1. ¿Qué es una Dangling Reference en Rust?**");
  println!("Una Dangling Reference ocurre cuando se intenta usar una referencia a un valor que ya no existe porque salió de su alcance (scope).");
  println!("Rust previene estas referencias asegurándose de que cualquier referencia válida siempre apunte a un valor que aún está en alcance.\n");

  // **2. Ejemplo básico de una Dangling Reference**
  println!("**2. Ejemplo básico de una Dangling Reference:**");
  {
      let r;
      {
          let x = 5; // x tiene su alcance dentro del bloque interno
          r = &x; // r intenta referenciar a x
      } // Aquí, x sale de alcance y es liberado
      // println!("r: {}", r); // Esto causaría un error de compilación
  }
  println!("Rust no permite compilar este código porque x no vive lo suficiente para r.\n");

  // **3. Ejemplo en funciones**
  println!("**3. Ejemplo de Dangling Reference en una función:**");
  fn longest(x: &str, y: &str) -> &str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

  // **Caso con Dangling Reference**
  println!("Intentando usar una referencia fuera de su alcance:");
  {
      let string1 = String::from("123456789");
      let result;
      {
          let string2 = String::from("abcdefghijklmnopqrstuvwxyz");
          result = longest(string1.as_str(), string2.as_str()); // string2 sale de alcance al final de este bloque
      }
      // println!("La cadena más larga es: {}", result); // Esto causaría un error
  }
  println!("Rust evita que este código se compile porque result intenta apuntar a string2, que ya no existe.\n");

  // **4. Cómo prevenir Dangling References**
  println!("**4. Prevención de Dangling References:**");
  println!("Rust utiliza un sistema llamado Borrow Checker que verifica los alcances de las referencias.");
  println!("- Asegúrate de que las referencias no vivan más tiempo que los valores que apuntan.");
  println!("- Usa lifetimes ('vidas') para ayudar al compilador a determinar los alcances correctos.");

  // **5. Ejemplo con lifetimes**
  println!("\n**5. Ejemplo con Lifetimes:**");
  fn safe_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

  let string1 = String::from("123456789");
  let string2 = String::from("abcdefghijklmnopqrstuvwxyz");
  let result = safe_longest(string1.as_str(), string2.as_str()); // Ambas referencias están en el mismo alcance
  println!("La cadena más larga es: {}", result); // Esto compila porque el compilador puede garantizar la seguridad

  // **6. Resumen**
  println!("\n**6. Resumen:**");
  println!("- Las Dangling References ocurren cuando una referencia apunta a un valor fuera de alcance.");
  println!("- Rust las previene usando el Borrow Checker, que asegura que las referencias sean válidas.");
  println!("- Usa lifetimes explícitas para manejar referencias más complejas y evitar errores.");
  println!("- Si un valor sale de alcance, no puede ser referenciado; Rust lo detectará y no permitirá compilar el código.");


  println!("¡Ejercicios completados correctamente!");

  println!("**Ejercicio 1: Detectar Dangling References**");
  println!("1. Intenta crear una referencia a un valor que sale de su alcance.");
  println!("   Observa cómo Rust impide compilar el código. (Ejemplo: una variable local que es referenciada fuera de su bloque).\n");

  println!("**Ejercicio 2: Prevenir Dangling References con Lifetimes**");
  println!("2. Escribe una función llamada `larger_string` que reciba dos referencias de cadenas (`&str`).");
  println!("   Usa lifetimes explícitas (`'a`) para garantizar que la referencia devuelta sea válida.\n");

  println!("**Ejercicio 3: Comparar Cadenas con Alcances Correctos**");
  println!("3. Declara dos cadenas en el mismo alcance y usa `larger_string` para encontrar la más larga.");
  println!("   Asegúrate de que el resultado sea seguro y válido.\n");

  println!("¡Completa estos ejercicios para entender y prevenir Dangling References en Rust!");
}
