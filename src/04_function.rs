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


  println!("**Ejercicio práctico: Funciones en Rust**");

  // **1. Función básica con retorno**
  println!("1. Escribe una función llamada `suma_doble` que tome dos parámetros enteros y retorne el doble de su suma.");
  println!("   Llama a la función con los valores 3 y 7 y muestra el resultado.\n");

  // **2. Función que retorna Unit type**
  println!("2. Define una función llamada `imprime_mensaje` que no reciba parámetros y solo imprima '¡Hola, Rust!'.");
  println!("   Llama a esta función y verifica que no retorna nada útil.\n");

  // **3. Última expresión como retorno**
  println!("3. Escribe una función llamada `maximo_mas_uno` que tome dos enteros como parámetros.");
  println!("   Retorna el mayor de los dos números más uno, usando solo una expresión al final de la función.\n");

  // **4. Bloques como expresiones**
  println!("4. Crea una función llamada `calcula_diferencia` que reciba dos enteros.");
  println!("   Usa un bloque dentro de la función para calcular y retornar la diferencia absoluta entre los números.\n");

  println!("¡Completa los ejercicios para reforzar los conceptos de funciones en Rust!");

}
