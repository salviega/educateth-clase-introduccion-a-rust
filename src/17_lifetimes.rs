fn main() {
  println!("**1. ¿Qué son los Lifetimes en Rust?**");
  println!("En Rust, cada referencia tiene un lifetime (tiempo de vida), que indica cuánto tiempo es válida.");
  println!("El compilador infiere automáticamente la mayoría de los lifetimes, pero en casos complejos, necesitamos especificarlos manualmente.");
  println!("Los lifetimes aseguran que nuestras referencias no apunten a datos que ya no existen.\n");

  // **2. Ejemplo básico sin anotaciones manuales**
  println!("**2. Ejemplo básico sin necesidad de anotaciones manuales:**");
  let string1 = String::from("Rust");
  let string2 = String::from("Programming");

  fn longer_without_lifetimes(x: &str, y: &str) -> &str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

  let result = longer_without_lifetimes(string1.as_str(), string2.as_str());
  println!("La cadena más larga es: {}\n", result);

  // **3. Caso que requiere anotaciones manuales**
  println!("**3. Caso que requiere anotaciones manuales:**");

  fn longer_with_lifetimes<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

  let result_with_lifetime = longer_with_lifetimes(string1.as_str(), string2.as_str());
  println!("Con lifetimes explícitos: {}\n", result_with_lifetime);

  // **4. Uso de lifetimes en estructuras**
  println!("**4. Uso de lifetimes en estructuras:**");

  struct MyEnum<'a> {
      greet: &'a str, // El campo 'greet' debe vivir al menos tanto como la instancia de `MyEnum`.
  }

  let hello = String::from("Hello, Rust!");
  let instance = MyEnum { greet: &hello }; // El lifetime de `hello` es mayor que el de `instance`.
  println!("Instancia de MyEnum: {}\n", instance.greet);

  // **5. Cómo evitar errores comunes con lifetimes**
  println!("**5. Cómo evitar errores comunes con lifetimes:**");

  println!("- Asegúrate de que las referencias sean válidas durante toda su vida útil.");
  println!("- Usa lifetimes explícitos solo cuando sea necesario.");
  println!("- Confía en el Borrow Checker para manejar lifetimes simples automáticamente.");
  println!("- Cuando trabajes con múltiples referencias, especifica cómo sus lifetimes están relacionadas.\n");

  // **6. Resumen**
  println!("**6. Resumen:**");
  println!("- Los lifetimes aseguran que las referencias no apunten a datos fuera de alcance.");
  println!("- En la mayoría de los casos, Rust infiere los lifetimes automáticamente.");
  println!("- Usa anotaciones explícitas (`'a`) cuando haya múltiples referencias con relaciones complejas.");
  println!("- Lifetimes son esenciales para escribir código seguro y eficiente en Rust.");

  

  println!("**Ejercicio 1: Identificar Lifetimes**");
  println!("1. Intenta escribir una función que retorne una referencia a un valor que salió de su alcance.");
  println!("   Observa cómo Rust impide compilar el código por problemas de lifetimes.\n");

  println!("**Ejercicio 2: Uso de Lifetimes en Funciones**");
  println!("2. Escribe una función llamada `mayor_lifetime` que reciba dos referencias de cadenas (`&str`).");
  println!("   Usa lifetimes explícitas (`'a`) para garantizar que la referencia devuelta sea válida.\n");

  println!("**Ejercicio 3: Lifetimes en Estructuras**");
  println!("3. Define una estructura llamada `Persona` con un campo `nombre` que sea una referencia de cadena.");
  println!("   Usa lifetimes explícitas para asegurar que `nombre` viva al menos tanto como la instancia de `Persona`.\n");

  println!("¡Completa estos ejercicios para entender cómo usar Lifetimes en Rust!");
}
