fn main() {
  println!("**1. ¿Qué es Borrowing en Rust?**");
  println!("Borrowing permite acceder a datos a través de referencias sin transferir el ownership.");
  println!("Las referencias se crean con `&` para referencias inmutables o `&mut` para referencias mutables.\n");

  // **2. Referencias inmutables (Immutable Borrowing)**
  println!("**2. Referencias inmutables:**");
  let s1 = String::from("hello"); // s1 es el propietario de la cadena
  let s2 = &s1; // s2 es una referencia inmutable a s1
  println!("s1: {}, s2: {}", s1, s2); // Ambas variables pueden usarse porque s2 no modifica s1

  println!("\nPodemos crear múltiples referencias inmutables:");
  let s3 = &s1;
  println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

  // **3. Referencias mutables (Mutable Borrowing)**
  println!("\n**3. Referencias mutables:**");
  let mut s4 = String::from("Rust");
  let s5 = &mut s4; // Creamos una referencia mutable
  s5.push_str(" es genial"); // Modificamos el contenido de s4 a través de s5
  println!("s5 modificó s4: {}", s5);

  println!("\nSolo puede existir una referencia mutable activa a la vez:");
  // let s6 = &mut s4; // Esto causará un error si no se comenta
  // println!("s4 con dos referencias mutables: {}", s6);

  // **4. Restricciones al combinar referencias mutables e inmutables**
  println!("\n**4. Restricciones de referencias mutables e inmutables:**");
  let mut s7 = String::from("¡Hola!");
  let r1 = &s7; // Referencia inmutable
  let r2 = &s7; // Otra referencia inmutable
  println!("Referencias inmutables: r1: {}, r2: {}", r1, r2);

  // let r3 = &mut s7; // Esto causará un error porque hay referencias inmutables activas
  // println!("r3: {}", r3);

  // **5. Funciones con referencias**
  println!("\n**5. Usando referencias en funciones:**");

  // Referencia inmutable como parámetro
  fn calculate_length(s: &String) -> usize {
      s.len() // Usamos `s` sin modificarlo
  }

  let s8 = String::from("Rustacean");
  let length = calculate_length(&s8);
  println!("La longitud de '{}' es {}.", s8, length);

  // Referencia mutable como parámetro
  fn change_string(s: &mut String) {
      s.push_str(", ¡bienvenido a Rust!"); // Modificamos el valor
  }

  let mut s9 = String::from("Hola");
  change_string(&mut s9); // Pasamos una referencia mutable
  println!("Después de modificar: {}", s9);

  // **6. Referencias colgantes (Dangling References)**
  println!("\n**6. Referencias colgantes:**");

  // Ejemplo que fallará si se descomenta
  // fn dangle() -> &String {
  //     let s = String::from("Hola"); // s se crea dentro de esta función
  //     &s // Intentamos devolver una referencia a s
  // } // Cuando s salga del alcance, se liberará su memoria, dejando una referencia inválida

  println!("Rust previene referencias colgantes compilando con errores si intentamos crearlas.");

  println!("\n**Resumen:**");
  println!("- Borrowing permite usar referencias inmutables (`&`) o mutables (`&mut`).");
  println!("- Las referencias inmutables permiten múltiples accesos simultáneos.");
  println!("- Las referencias mutables permiten modificar datos, pero solo puede existir una activa.");
  println!("- Rust garantiza la seguridad evitando referencias colgantes o conflictos entre referencias mutables e inmutables.\n");



  println!("**Solución: Borrowing en Rust**");

  // **1. Referencias inmutables**
  let s1 = String::from("Hola, Rust!");
  let r1 = &s1; // Primera referencia inmutable
  let r2 = &s1; // Segunda referencia inmutable
  println!("r1: {}, r2: {}", r1, r2); // Ambas referencias pueden usarse

  // **2. Referencias mutables**
  let mut s2 = String::from("Rust es increíble");
  let r3 = &mut s2; // Referencia mutable
  r3.push_str(" y poderoso."); // Modifica el contenido del String
  println!("Después de modificar: {}", r3);

  // **3. Restricciones de referencias**
  let mut s3 = String::from("Restricciones");
  let r4 = &s3; // Referencia inmutable
  let r5 = &s3; // Otra referencia inmutable
  println!("r4: {}, r5: {}", r4, r5);
  // let r6 = &mut s3; // Esto causa un error: no puede coexistir con referencias inmutables
  println!("No se puede tener una referencia mutable mientras existan referencias inmutables.");

  // **4. Referencias en funciones**
  fn obtener_longitud(s: &String) -> usize {
      s.len() // Retorna la longitud del String
  }

  fn añadir_texto(s: &mut String) {
      s.push_str(" ¡Bienvenido a Rust!"); // Modifica el String
  }

  let s4 = String::from("Rustacean");
  let longitud = obtener_longitud(&s4);
  println!("La longitud de '{}' es {}.", s4, longitud);

  let mut s5 = String::from("Hola");
  añadir_texto(&mut s5); // Pasa una referencia mutable
  println!("Después de añadir texto: {}", s5);

  println!("¡Ejercicio completado correctamente!");

}
