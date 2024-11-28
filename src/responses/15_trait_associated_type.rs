fn main() {
  println!("**1. ¿Qué son los Associated Types en Rust?**");
  println!("Los Associated Types son una forma de usar parámetros de tipo dentro de Traits.");
  println!("Permiten que los Traits sean más flexibles y reutilizables al no requerir que los tipos específicos se definan al momento de declarar el Trait.");
  println!("Es como un juego de 'llenar los espacios en blanco': el Trait define los espacios en blanco, y los tipos que lo implementan llenan esos espacios con tipos concretos.\n");

  // **2. Ejemplo básico del Trait `Iterator`**
  println!("**2. Ejemplo básico del Trait `Iterator`:**");
  println!("El Trait `Iterator` de la librería estándar usa un Associated Type llamado `Item` para definir el tipo de datos que recorrerá.");

  pub trait Iterator {
      type Item; // Tipo asociado

      fn next(&mut self) -> Option<Self::Item>; // Método que retorna el tipo asociado envuelto en `Option`
  }

  println!("Un tipo que implemente `Iterator` puede especificar su propio tipo `Item`.");
  println!("Por ejemplo, si el iterador produce valores de tipo `i32`, el método `next` devolverá un `Option<i32>`.\n");

  // **3. Definir un Trait con un Associated Type**
  println!("**3. Definir un Trait con un Associated Type:**");

  trait Summary {
      type Output; // Tipo asociado

      fn summarize(&self) -> Self::Output; // Método que retorna el tipo asociado
  }

  println!("El Trait `Summary` permite que diferentes tipos definan su propia salida (`Output`).\n");

  // **4. Implementar un Trait con Associated Types**
  println!("**4. Implementar un Trait con Associated Types:**");

  struct NewsArticle {
      headline: String,
      location: String,
      author: String,
  }

  impl Summary for NewsArticle {
      type Output = String; // Especificamos el tipo concreto para `Output`

      fn summarize(&self) -> Self::Output {
          format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
  }

  let article = NewsArticle {
      headline: String::from("¡Los pingüinos ganan el campeonato de la Stanley Cup!"),
      location: String::from("Pittsburgh, PA, USA"),
      author: String::from("Iceburgh"),
  };

  println!("Resumen del artículo: {}", article.summarize());

  // **5. Ventajas de Associated Types**
  println!("\n**5. Ventajas de Associated Types:**");
  println!("- Permiten que un Trait use un tipo abstracto que puede ser concretado al implementarlo.");
  println!("- Reducen la complejidad de usar genéricos al hacer que los métodos sean más claros y concisos.");
  println!("- Facilitan compartir el mismo comportamiento entre múltiples tipos concretos.\n");

  // **6. Resumen**
  println!("**Resumen:**");
  println!("- Los Associated Types son parámetros de tipo definidos dentro de un Trait.");
  println!("- Se concretan al implementar el Trait para un tipo específico.");
  println!("- Usarlos permite que los Traits sean más flexibles y fáciles de usar.");
  println!("- Ejemplo: `Iterator` usa un Associated Type llamado `Item` para definir el tipo de valores que recorrerá.");

  trait Operaciones {
    type Resultado;

    fn calcular(&self) -> Self::Resultado;
}

struct Suma {
    a: i32,
    b: i32,
}

impl Operaciones for Suma {
    type Resultado = i32;

    fn calcular(&self) -> Self::Resultado {
        self.a + self.b
    }
}

struct Concatenar {
    x: String,
    y: String,
}

impl Operaciones for Concatenar {
    type Resultado = String;

    fn calcular(&self) -> Self::Resultado {
        format!("{}{}", self.x, self.y)
    }
}

fn main() {
    println!("**Solución: Associated Types en Rust**");

    // **Ejercicio 2: Implementar el Trait en Suma**
    let suma = Suma { a: 10, b: 20 };
    println!("El resultado de la suma es: {}", suma.calcular());

    // **Ejercicio 3: Implementar el Trait en Concatenar**
    let concatenar = Concatenar {
        x: String::from("Hola, "),
        y: String::from("Rust!"),
    };
    println!("El resultado de la concatenación es: {}", concatenar.calcular());
}
}
