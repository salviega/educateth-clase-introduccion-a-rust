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


  println!("**Solución: Lifetimes en Rust**");

  // **Ejercicio 1: Identificar Lifetimes**
  println!("\n**Ejercicio 1: Identificar Lifetimes**");
  println!("Este código muestra cómo Rust impide referencias colgantes:");
  // Descomentar este código causará un error de compilación.
  // fn dangling_reference<'a>() -> &'a str {
  //     let local_string = String::from("Hola"); // Variable local
  //     &local_string // Intentamos retornar una referencia a un valor que saldrá de alcance
  // }

  // **Ejercicio 2: Uso de Lifetimes en Funciones**
  println!("\n**Ejercicio 2: Uso de Lifetimes en Funciones**");

  fn mayor_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

  let cadena1 = "Rust";
  let cadena2 = "Lenguaje de programación";
  let resultado = mayor_lifetime(cadena1, cadena2);
  println!("La cadena más larga es: {}", resultado);

  // **Ejercicio 3: Lifetimes en Estructuras**
  println!("\n**Ejercicio 3: Lifetimes en Estructuras**");

  struct Persona<'a> {
      nombre: &'a str,
  }

  let nombre = String::from("Juan Pérez");
  let persona = Persona { nombre: &nombre }; // El lifetime de `nombre` debe ser mayor o igual al de `persona`
  println!("Nombre de la persona: {}", persona.nombre);

  println!("\n¡Ejercicios completados correctamente!");
}
