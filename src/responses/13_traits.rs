fn main() {
  println!("**1. ¿Qué son los Traits en Rust?**");
  println!("Los Traits son una forma de definir comportamientos compartidos entre diferentes tipos.");
  println!("Se usan para garantizar que varios tipos puedan implementar métodos con el mismo nombre, pero con diferentes lógicas.");
  println!("Un Trait es como una interfaz: define qué métodos deben implementar los tipos que 'tengan' este Trait.\n");

  // **2. Definir un Trait**
  println!("**2. Definir un Trait:**");
  trait MigrantBird {
      fn migrate(&self) -> String; // Método sin implementación por defecto
  }
  println!("Definimos el Trait 'MigrantBird' con el método 'migrate'. Este método debe implementarse en cada tipo.\n");

  // **3. Implementación básica en una estructura**
  println!("**3. Implementación básica en una estructura:**");
  struct WildGoose {
      color: String,
  }

  impl WildGoose {
      fn new() -> Self {
          WildGoose {
              color: "gray".to_string(),
          }
      }

      fn inhabit(&self) {
          println!("Los gansos salvajes descansan junto al lago.");
      }
  }

  impl MigrantBird for WildGoose {
      fn migrate(&self) -> String {
          "Los gansos vuelan en formación en V.".to_string()
      }
  }

  let goose = WildGoose::new();
  goose.inhabit();
  println!("{}", goose.migrate());

  // **4. Implementación en otro tipo**
  println!("\n**4. Implementación en otro tipo:**");
  struct Swallow {
      color: String,
  }

  impl Swallow {
      fn new() -> Self {
          Swallow {
              color: "black".to_string(),
          }
      }

      fn build_nest(&self) {
          println!("Las golondrinas construyen nidos bajo los aleros.");
      }
  }

  impl MigrantBird for Swallow {
      fn migrate(&self) -> String {
          "Las golondrinas vuelan rápido, pero necesitan descansar con frecuencia.".to_string()
      }
  }

  let swallow = Swallow::new();
  swallow.build_nest();
  println!("{}", swallow.migrate());

  // **5. Usar Traits en funciones**
  println!("\n**5. Usar Traits en funciones:**");
  println!("Podemos escribir funciones que trabajen con cualquier tipo que implemente un Trait.");

  fn describe_migration(bird: &impl MigrantBird) {
      println!("Migración: {}", bird.migrate());
  }

  describe_migration(&goose);
  describe_migration(&swallow);

  // **6. Implementaciones por defecto**
  println!("\n**6. Implementaciones por defecto:**");
  println!("Los Traits pueden tener implementaciones por defecto para métodos.");
  trait Bird {
      fn sound(&self) -> String {
          "Cualquier pájaro puede hacer un sonido.".to_string()
      }
  }

  impl Bird for WildGoose {}
  impl Bird for Swallow {}

  println!("Sonido del ganso: {}", goose.sound());
  println!("Sonido de la golondrina: {}", swallow.sound());

  // **7. Resumen**
  println!("\n**Resumen:**");
  println!("- Los Traits son similares a interfaces y definen comportamientos compartidos.");
  println!("- Permiten implementar métodos con diferentes lógicas para tipos distintos.");
  println!("- Se pueden usar en funciones para trabajar con cualquier tipo que implemente un Trait.");
  println!("- Pueden incluir métodos con implementaciones por defecto.");

  println!("**Solución: Traits en Rust**");

    // **1. Definir un Trait**
    trait Vehicle {
        fn drive(&self);
        fn stop(&self);

        // Implementación por defecto
        fn fuel_type(&self) -> String {
            "Gasolina".to_string()
        }
    }

    // **2. Implementar el Trait en estructuras**
    struct Car {
        model: String,
    }

    struct Bicycle {
        type_of_bike: String,
    }

    impl Vehicle for Car {
        fn drive(&self) {
            println!("El coche modelo {} está en movimiento.", self.model);
        }

        fn stop(&self) {
            println!("El coche modelo {} se ha detenido.", self.model);
        }

        fn fuel_type(&self) -> String {
            "Diésel".to_string()
        }
    }

    impl Vehicle for Bicycle {
        fn drive(&self) {
            println!("La bicicleta tipo {} está en movimiento.", self.type_of_bike);
        }

        fn stop(&self) {
            println!("La bicicleta tipo {} se ha detenido.", self.type_of_bike);
        }
    }

    // **3. Usar el Trait en funciones**
    fn start_trip(vehicle: &impl Vehicle) {
        vehicle.drive();
        vehicle.stop();
    }

    let car = Car {
        model: "Toyota Corolla".to_string(),
    };

    let bicycle = Bicycle {
        type_of_bike: "Montaña".to_string(),
    };

    println!("\nIniciando viaje con el coche:");
    start_trip(&car);

    println!("\nIniciando viaje con la bicicleta:");
    start_trip(&bicycle);

    // **4. Implementaciones por defecto**
    println!("\nTipo de combustible del coche: {}", car.fuel_type());
    println!("Tipo de combustible de la bicicleta: {}", bicycle.fuel_type());
}
