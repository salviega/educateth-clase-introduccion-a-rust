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


  println!("**Ejercicio 1: Associated Types en Traits**");
  println!("1. Define un Trait llamado `Operaciones` con un tipo asociado `Resultado`.");
  println!("   El Trait debe incluir un método llamado `calcular` que retorne un valor del tipo asociado.\n");

  println!("**Ejercicio 2: Implementar el Trait en una Estructura**");
  println!("2. Crea una estructura llamada `Suma` con dos campos de tipo `i32`.");
  println!("   Implementa el Trait `Operaciones` para `Suma`, usando el tipo asociado `Resultado` como `i32`.");
  println!("   El método `calcular` debe retornar la suma de los dos campos.\n");

  println!("**Ejercicio 3: Implementar el Trait en Otro Tipo**");
  println!("3. Crea una estructura llamada `Concatenar` con dos campos de tipo `String`.");
  println!("   Implementa el Trait `Operaciones` para `Concatenar`, usando el tipo asociado `Resultado` como `String`.");
  println!("   El método `calcular` debe retornar la concatenación de los dos campos.\n");

  println!("¡Completa estos ejercicios para practicar los Associated Types en Rust!");
}
