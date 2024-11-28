fn main() {
  println!("**1. ¿Qué son los statements y expressions en Rust?**");

  // **Statement: Unidad de ejecución que no retorna un valor**
  let a = 5; // Statement: Define una variable, no retorna un valor
  println!("Statement: let a = 5; no retorna nada, solo ejecuta la asignación.");

  // **Intentar usar un statement como una expresión causa un error**
  // let b = (let c = 3); // Error: Un statement no retorna un valor

  // **Expression: Unidad computacional que calcula y retorna un valor**
  let y = {
      let x = 3; // Statement dentro de un bloque
      x + 1       // Expression: Calcula x + 1 y retorna el valor
  };
  println!("Expression: Bloque computacional retornó y = {}", y);

  // **Unit Type `()` como resultado de un statement**
  let z = {
      let _ = 2 + 2; // Esto es un statement, no retorna valor (solo `()`)
  };
  println!("El resultado de un bloque statement es unit type `z = {:?}`\n", z);

  println!("¡Los statements realizan acciones, y las expressions calculan valores!");


  println!("**Ejercicio práctico: Statements y Expressions en Rust**");

  // **1. Variables**
  println!("1. Declara una variable `x` como un statement que asigne el valor 10.");
  println!("   Luego, crea una expresión que sume 5 a `x` y almacénala en una nueva variable `y`.\n");

  // **2. Bloques**
  println!("2. Escribe un bloque que calcule el cuadrado de `y` y retorne el valor.");
  println!("   Asigna este valor a una nueva variable `z`.\n");

  // **3. Unit type**
  println!("3. Declara una variable `resultado` que contenga un bloque de código que no retorne nada.");
  println!("   Muestra el valor de `resultado` para verificar que sea `()`.\n");

  println!("¡Completa estos ejercicios para comprender mejor los statements y expressions en Rust!");

  println!("**Solución: Statements y Expressions en Rust**");

    // **1. Variables**
    let x = 10; // Statement
    let y = x + 5; // Expression
    println!("x = {}, y = {}", x, y);

    // **2. Bloques**
    let z = {
        y * y // Expression dentro de un bloque que calcula el cuadrado
    };
    println!("z (cuadrado de y) = {}", z);

    // **3. Unit type**
    let resultado = {
        println!("Este bloque no retorna nada.");
        // Sin expresión al final, el bloque retorna `()`
    };
    println!("resultado (unit type) = {:?}", resultado);

    println!("¡Ejercicio resuelto correctamente!");
}
