fn main() {
    // **1. Inmutabilidad por defecto**
    println!("1. Inmutabilidad por defecto:");
    let x = 10; // Variable inmutable
    println!("x = {}", x);
    // x = 20; // Error: x no es mutable
    println!("Nota: Por defecto, las variables declaradas con 'let' son inmutables.\n");

    // **2. Mutabilidad con 'mut'**
    println!("2. Mutabilidad con 'mut':");
    let mut y = 15; // Variable mutable
    println!("y antes = {}", y);
    y = 20; // Reasignación válida
    println!("y después = {}\n", y);

    // **3. Strings mutables y operaciones**
    println!("3. Strings mutables y operaciones:");
    let mut greeting = String::from("Hello");
    println!("Greeting inicial: {}", greeting);
    greeting.push_str(", world!"); // Modificar contenido
    println!("Greeting modificado: {}\n", greeting);

    // **4. Diferencia entre `let` y `const`**
    println!("4. Diferencia entre `let` y `const`:");
    const PI: f64 = 3.14159; // Constante
    println!("PI (constante): {}", PI);
    let z = 42; // Variable
    println!("z (variable): {}", z);
    // z = 100; // Si z no es mutable, esto daría un error.
    println!("Nota: 'const' siempre es inmutable y requiere tipo explícito.\n");

    // **5. Variables globales con 'static'**
    println!("5. Variables globales con 'static':");
    static GREETING: &str = "Hello, global world!";
    println!("GREETING: {}", GREETING);

    unsafe {
        static mut COUNTER: u32 = 0; // Variable global mutable
        COUNTER += 1;
        println!("COUNTER modificado: {}\n", COUNTER);
    }

    // **6. Sombras de variables (Shadowing)**
    println!("6. Sombras de variables (Shadowing):");
    let x = 5; // Primera declaración de x
    println!("x inicial: {}", x);
    let x = x + 1; // Redeclaración (sombra)
    println!("x después de sombra: {}", x);
    let x = "Texto"; // Redeclaración con nuevo tipo
    println!("x con nuevo tipo: {}\n", x);

    // **7. Comparación práctica de Strings y &str**
    println!("7. Comparación práctica de Strings y &str:");
    let string_literal: &str = "Hello, slice!"; // `&str`
    let mut owned_string: String = String::from("Hello, owned!"); // `String`
    println!("&str (inmutable): {}", string_literal);
    println!("String antes de modificar: {}", owned_string);
    owned_string.push_str(" Modified!"); // Modificando `String`
    println!("String después de modificar: {}\n", owned_string);

    // **8. Ejemplo final: Tipos escalares y compuestos**
    println!("8. Ejemplo final: Tipos escalares y compuestos:");
    let a: i32 = 42; // Entero
    let b: f64 = 3.14; // Flotante
    let c: bool = true; // Booleano
    let d: char = 'R'; // Carácter
    let tuple: (i32, f64, char) = (a, b, d); // Tupla
    let array: [i32; 3] = [1, 2, 3]; // Array

    println!("Entero: {}", a);
    println!("Flotante: {}", b);
    println!("Booleano: {}", c);
    println!("Carácter: {}", d);
    println!("Tupla: {:?}", tuple);
    println!("Array: {:?}\n", array);

    println!("¡Todo listo para entender mutabilidad e inmutabilidad en Rust!");

    /* //////////////////////////////// */
    
    // **9. Ejercicio práctico**
    println!("9. Ejercicio práctico:");
    println!("Resuelve el siguiente problema:");
    println!("1. Declara una constante con el valor de tu edad.");
    println!("2. Declara una variable mutable con tu nombre y modifícala para que contenga tu nombre completo.");
    println!("3. Crea una tupla que contenga tu edad, tu altura (en metros) y la inicial de tu primer nombre.");
    println!("4. Crea un array que contenga tres valores de tu elección (números o cadenas) y muéstralo.");
    println!("5. Usa shadowing para modificar la edad sumándole un año.");
    println!("6. Usa una variable 'static' para almacenar tu saludo personalizado y modifícalo en un bloque 'unsafe'.\n");

    println!("¡Inténtalo y experimenta con el código para entender mejor los conceptos!");
}
