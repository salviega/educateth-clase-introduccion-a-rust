fn main() {
  println!("**1. ¿Qué es Ownership en Rust?**");
  println!("Ownership es el mecanismo de Rust para gestionar memoria de manera segura y eficiente, sin necesitar un garbage collector.");
  println!("Cada valor tiene un único propietario (variable), y cuando ese propietario sale del alcance, el valor es automáticamente liberado.\n");

  println!("**2. Principios básicos del Ownership:**");
  println!("1. Cada valor en Rust tiene una variable como propietario.");
  println!("2. Solo puede haber un propietario a la vez.");
  println!("3. Cuando el propietario sale del alcance, el valor se elimina automáticamente.\n");

  println!("**3. Ejemplo básico de Ownership:**");
  {
      let s1 = String::from("hello"); // s1 es el propietario del String
      println!("s1 es el propietario del valor: {}", s1);
  } // Aquí, s1 sale del alcance, y su memoria se libera automáticamente.
  println!("Cuando s1 sale del alcance, Rust libera automáticamente su memoria.\n");

  println!("**4. Transferencia de Ownership (Move):**");
  let s1 = String::from("hola");
  println!("s1 es el propietario del valor: {}", s1);
  let s2 = s1; // Ownership de s1 se transfiere a s2
  println!("Ownership de s1 ahora pertenece a s2: {}", s2);

  // Si intentamos usar s1, obtendremos un error:
  // println!("Intentando usar s1: {}", s1); // Esto causará un error de compilación
  println!("Nota: s1 ya no es válido después de la transferencia de ownership.\n");

  println!("**5. Borrowing (Préstamos) con referencias:**");
  let s1 = String::from("mundo");
  let s2 = &s1; // Creamos una referencia a s1 (ownership no cambia)
  println!("s1: {}, s2: {}", s1, s2); // Ambas variables pueden usarse
  println!("Nota: Las referencias permiten leer sin transferir el ownership.\n");

  println!("**6. Mutabilidad y Ownership:**");
  let mut s1 = String::from("adiós");
  let s2 = &mut s1; // Creamos una referencia mutable
  s2.push_str(", Rust!"); // Modificamos el valor a través de la referencia
  println!("s2 modificó s1: {}", s2);
  println!("Nota: Solo puede haber una referencia mutable activa a la vez.\n");

  println!("**7. Restricciones de Borrowing (Préstamos):**");
  let mut s1 = String::from("restricciones");
  let s2 = &s1; // Referencia inmutable
  let s3 = &s1; // Otra referencia inmutable
  println!("s2: {}, s3: {}", s2, s3);
  // let s4 = &mut s1; // Esto causará un error: no se puede tener referencia mutable junto a referencias inmutables
  println!("Nota: Rust no permite referencias mutables mientras existen referencias inmutables activas.\n");

  println!("**8. Resumen:**");
  println!("Ownership en Rust permite gestionar memoria de manera segura:");
  println!("- Los valores tienen un único propietario.");
  println!("- El ownership puede transferirse (Move).");
  println!("- Las referencias permiten compartir acceso sin transferir ownership.");
  println!("- Rust garantiza que no haya data races mediante restricciones claras.\n");

  println!("¡Ahora entiendes cómo funciona Ownership en Rust!");


  println!("**Ejercicio práctico: Ownership en Rust**");

  // **1. Crear y transferir Ownership**
  println!("1. Declara una variable que contenga un String y transfiere su ownership a otra variable.");
  println!("   Luego, intenta usar la variable original y observa lo que sucede.\n");

  // **2. Borrowing (Préstamos)**
  println!("2. Crea una función que tome una referencia como parámetro y la use para imprimir el valor.");
  println!("   Llama a la función con una referencia a un String.\n");

  // **3. Mutabilidad y referencias mutables**
  println!("3. Declara un String mutable y crea una referencia mutable para modificar su contenido.");
  println!("   Asegúrate de no tener referencias inmutables activas al mismo tiempo.\n");

  // **4. Restricciones de Borrowing**
  println!("4. Declara un String y crea dos referencias inmutables.");
  println!("   Intenta crear una referencia mutable mientras las referencias inmutables están activas y observa el error.\n");

  println!("¡Resuelve estos ejercicios para comprender cómo Ownership, Borrowing y mutabilidad funcionan en Rust!");
}
