fn main() {
  println!("**1. ¿Qué es una función en Rust?**");
  println!("Una función es un bloque reutilizable de código que realiza una tarea específica. Puede:");
  println!("- Tomar parámetros de entrada.");
  println!("- Realizar operaciones.");
  println!("- Retornar valores (opcionalmente).\n");

  // **Declarar una función básica**
  fn suma(a: i32, b: i32) -> i32 {
      a + b // Retorna la suma como una expresión
  }

  let resultado = suma(5, 10);
  println!("Llamada a 'suma(5, 10)': {}", resultado);

  println!("\n**2. Snake_case en nombres de funciones:**");
  println!("En Rust, los nombres de funciones siguen la convención 'snake_case'.");
  println!("Ejemplo: 'fn mi_funcion()' es correcto según las convenciones de Rust.\n");

  // **Funciones con diferentes tipos de retorno**
  println!("**3. Diferentes tipos de retorno:**");

  // **Función que retorna un valor**
  fn max_plus_one(x: i32, y: i32) -> i32 {
      if x > y {
          return x + 1; // Retorna usando la palabra clave 'return'
      }
      y + 1 // Última expresión del bloque, sin punto y coma
  }

  let max_resultado = max_plus_one(8, 5);
  println!("'max_plus_one(8, 5)': {}", max_resultado);

  // **Función que retorna Unit type `()`**
  fn print_hello() {
      println!("hello"); // Esta función no tiene retorno explícito, retorna `()`
  }

  print_hello(); // Llama a la función y no retorna un valor útil

  // **Función divergente (que nunca retorna)**
  fn diverging() -> ! {
      panic!("Esta función termina el programa y nunca retorna.");
  }

  // Descomenta para probar el comportamiento de una función divergente.
  // diverging();

  println!("\n**4. Parámetros en funciones:**");
  println!("Los parámetros siempre deben declararse con tipos explícitos.");
  println!("Ejemplo: 'fn suma(a: i32, b: i32) -> i32'.");
  println!("Rust no infiere tipos de parámetros en funciones.\n");

  // **Bloques como expresiones en funciones**
  println!("**5. Bloques como expresiones en funciones:**");

  fn calcula_cuadrado_y_cubo(x: i32) -> (i32, i32) {
      let cuadrado = x * x;
      let cubo = {
          let cuadrado_local = cuadrado; // Reutiliza el valor de cuadrado
          cuadrado_local * x
      };
      (cuadrado, cubo) // Retorna una tupla
  }

  let (cuadrado, cubo) = calcula_cuadrado_y_cubo(3);
  println!("'calcula_cuadrado_y_cubo(3)': cuadrado = {}, cubo = {}\n", cuadrado, cubo);

  println!("**6. Características únicas de las funciones en Rust:**");
  println!("- Siempre retornan valores, incluso si el valor es el unit type `()`.");
  println!("- Usan la última expresión del cuerpo como valor de retorno.");
  println!("- Los parámetros y tipos de retorno son explícitos, lo que mejora la seguridad del código.");
  println!("- Pueden ser divergentes, marcadas con el tipo especial `!` si nunca retornan.");


  println!("**Solución: Funciones en Rust**");

  // **1. Función básica con retorno**
  fn suma_doble(a: i32, b: i32) -> i32 {
      2 * (a + b) // Retorna el doble de la suma
  }
  let resultado_suma_doble = suma_doble(3, 7);
  println!("'suma_doble(3, 7)' = {}", resultado_suma_doble);

  // **2. Función que retorna Unit type**
  fn imprime_mensaje() {
      println!("¡Hola, Rust!");
  }
  imprime_mensaje(); // Llama a la función que no retorna nada útil

  // **3. Última expresión como retorno**
  fn maximo_mas_uno(x: i32, y: i32) -> i32 {
      if x > y {
          x + 1
      } else {
          y + 1
      } // Última expresión decide el retorno
  }
  let resultado_maximo = maximo_mas_uno(4, 9);
  println!("'maximo_mas_uno(4, 9)' = {}", resultado_maximo);

  // **4. Bloques como expresiones**
  fn calcula_diferencia(a: i32, b: i32) -> i32 {
      let diferencia = {
          if a > b {
              a - b
          } else {
              b - a
          }
      };
      diferencia // Retorna la diferencia absoluta
  }
  let resultado_diferencia = calcula_diferencia(10, 6);
  println!("'calcula_diferencia(10, 6)' = {}", resultado_diferencia);

  println!("¡Ejercicio resuelto correctamente!");

}
