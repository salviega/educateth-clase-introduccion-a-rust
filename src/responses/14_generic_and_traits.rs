fn main() {
  println!("**1. ¿Qué son los tipos genéricos y Traits en Rust?**");
  println!("Los tipos genéricos permiten escribir código reutilizable que funciona con múltiples tipos.");
  println!("Los Traits definen comportamientos que los tipos deben cumplir, como comparaciones o clonación.");
  println!("En combinación, generics y traits permiten que el código sea flexible y eficiente.\n");

  // **2. Ejemplo básico sin genéricos**
  println!("**2. Problema sin genéricos:**");
  println!("Queremos encontrar el elemento más grande en una lista. Sin genéricos, necesitamos repetir código.");
  
  fn largest_i32(list: &[i32]) -> i32 {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  fn largest_i64(list: &[i64]) -> i64 {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  let nums_i32 = [1, 2, 3];
  let nums_i64 = [10, 20, 30];

  println!("Mayor (i32): {}", largest_i32(&nums_i32));
  println!("Mayor (i64): {}", largest_i64(&nums_i64));

  // **3. Usando genéricos**
  println!("\n**3. Usando genéricos:**");
  println!("Con genéricos, podemos evitar la duplicación de código.");
  
  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  let nums_f64 = [3.1, 7.2, 9.5];
  println!("Mayor (f64): {}", largest(&nums_f64));

  // **4. Monomorfización**
  println!("\n**4. Monomorfización:**");
  println!("Rust compila genéricos generando funciones específicas para cada tipo usado.");
  println!("Por ejemplo, el código con genéricos se transforma en funciones específicas como estas:");
  println!("1. Para i32: `fn largest_i32`");
  println!("2. Para i64: `fn largest_i64`");
  println!("Esto asegura que no haya impacto en el rendimiento.\n");

  // **5. Traits como restricciones**
  println!("**5. Traits como restricciones:**");
  println!("Para usar el operador '>' en genéricos, el tipo debe implementar el Trait `PartialOrd`.");
  println!("Si necesitamos copiar valores, el tipo también debe implementar `Copy`.");

  fn largest_with_constraints<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  let nums_char = ['a', 'b', 'c'];
  println!("Mayor (char): {}", largest_with_constraints(&nums_char));

  // **6. Sintaxis `where`**
  println!("\n**6. Usar `where` para restricciones:**");
  println!("Podemos usar `where` para hacer más legible el código con múltiples restricciones.");

  fn largest_where<T>(list: &[T]) -> T
  where
      T: PartialOrd + Copy,
  {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
      largest
  }

  println!("Mayor (f64 usando where): {}", largest_where(&nums_f64));

  // **7. Resumen**
  println!("\n**Resumen:**");
  println!("- Los genéricos permiten escribir funciones y estructuras reutilizables para múltiples tipos.");
  println!("- Los Traits, como `PartialOrd` y `Copy`, definen comportamientos que los tipos deben cumplir.");
  println!("- Rust usa monomorfización para compilar genéricos en funciones específicas para cada tipo.");
  println!("- Usar genéricos y Traits juntos mejora la flexibilidad del código sin afectar el rendimiento.");


  println!("**Solución: genéricos y Traits**");

// **Ejercicio 1**
println!("**Solución: Encontrar el Mayor Número**");
fn mayor<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

let numeros = [1, 4, 2, 8, 5];
let maximo = mayor(&numeros);
println!("El número mayor es: {}", maximo);

// **Ejercicio 2**
println!("\n**Solución: Intercambiar Valores de un Par**");
struct Par<T> {
    a: T,
    b: T,
}

impl<T> Par<T> {
    fn intercambiar(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
    }
}

let mut par = Par { a: 10, b: 20 };
println!("Antes del intercambio: a = {}, b = {}", par.a, par.b);
par.intercambiar();
println!("Después del intercambio: a = {}, b = {}", par.a, par.b);

// **Ejercicio 3**
println!("\n**Solución: Comparar Números con un Trait**");
trait Comparar {
    fn es_mayor(&self, otro: &Self) -> bool;
}

impl Comparar for i32 {
    fn es_mayor(&self, otro: &i32) -> bool {
        self > otro
    }
}

let num1 = 10;
let num2 = 5;

if num1.es_mayor(&num2) {
    println!("{} es mayor que {}", num1, num2);
} else {
    println!("{} no es mayor que {}", num1, num2);
}

}
