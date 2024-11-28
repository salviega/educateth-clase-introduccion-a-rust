fn main() {
  println!("**1. ¿Qué son los métodos en Rust?**");
  println!("Los métodos son funciones asociadas con tipos específicos, como structs o enums.");
  println!("Se definen dentro de un bloque `impl` y el primer parámetro es siempre `&self`, que representa la instancia.");
  println!("Los métodos permiten definir comportamientos directamente asociados a las propiedades de un tipo.\n");

  // **2. Ejemplo básico con struct y método**
  println!("**2. Ejemplo básico con struct y método:**");
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }

  let rect1 = Rectangle { width: 30, height: 50 };
  println!(
      "El área del rectángulo es {} píxeles cuadrados.",
      rect1.area() // Llamada al método
  );

  // **3. Métodos con el mismo nombre que los campos**
  println!("\n**3. Métodos con el mismo nombre que los campos:**");
  impl Rectangle {
      pub fn width(&self) -> u32 {
          self.width
      }
  }

  println!("El ancho del rectángulo es: {}", rect1.width());

  // **4. Uso de funciones asociadas**
  println!("\n**4. Uso de funciones asociadas:**");
  impl Rectangle {
      pub fn new(width: u32, height: u32) -> Self {
          Rectangle { width, height }
      }
  }

  let rect2 = Rectangle::new(10, 20);
  println!(
      "Rectángulo creado con una función asociada: width = {}, height = {}",
      rect2.width, rect2.height
  );

  // **5. Métodos que no usan `self` como primer parámetro**
  println!("\n**5. Métodos que no usan `self` como primer parámetro:**");
  impl Rectangle {
      pub fn is_square(width: u32, height: u32) -> bool {
          width == height
      }
  }

  let is_square = Rectangle::is_square(10, 10);
  println!("¿Es un cuadrado?: {}", is_square);

  // **6. Resumen**
  println!("\n**Resumen:**");
  println!("- Los métodos definen comportamientos asociados a structs, enums o traits.");
  println!("- Se usan `impl` y el primer parámetro es `&self` para acceder a las propiedades.");
  println!("- Las funciones asociadas (sin `self`) se usan para inicializar o calcular algo que no depende de una instancia.");
  println!("- Los métodos hacen que el código sea más claro, legible y fácil de mantener.");

  println!("**Solución: Métodos en Rust**");

  // **1. Definir un struct y métodos básicos**
  struct Circle {
      radius: f64,
  }

  impl Circle {
      // Método para calcular el área
      fn area(&self) -> f64 {
          std::f64::consts::PI * self.radius * self.radius
      }

      // **2. Función asociada**
      // Función asociada para crear una nueva instancia de Circle
      fn new(radius: f64) -> Self {
          Circle { radius }
      }

      // **3. Métodos adicionales**
      // Método para calcular el diámetro
      fn diameter(&self) -> f64 {
          self.radius * 2.0
      }

      // **4. Métodos sin `self` como primer parámetro**
      // Función asociada para comparar radios
      fn is_larger(radius1: f64, radius2: f64) -> bool {
          radius1 > radius2
      }
  }

  let circle1 = Circle::new(5.0);
  println!("Área del círculo: {:.2}", circle1.area());
  println!("Diámetro del círculo: {:.2}", circle1.diameter());

  let circle2 = Circle::new(3.0);
  let larger = Circle::is_larger(circle1.radius, circle2.radius);
  println!("¿El círculo1 es más grande que el círculo2?: {}", larger);

  println!("¡Ejercicio completado correctamente!");
}
