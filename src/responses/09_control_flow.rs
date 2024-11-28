fn main() {
  println!("**1. Introducción a Control Flow en Rust**");
  println!("El control de flujo permite decidir cómo se ejecuta el programa basándose en condiciones y bucles.");
  println!("Rust ofrece herramientas como `if`, `for`, `while` y `loop` para construir la lógica del programa.\n");

  // **2. If-else como expresión**
  println!("**2. If-else como expresión:**");
  let condition = true;
  let number = if condition { 5 } else { 6 }; // `if` retorna un valor basado en la condición
  println!("El valor de 'number' es: {}", number);

  // **3. For loop básico**
  println!("\n**3. For loop básico:**");
  for i in 1..=5 { // Iteramos desde 1 hasta 5 (inclusive)
      println!("Valor: {}", i);
  }

  // **4. While loop**
  println!("\n**4. While loop:**");
  let mut counter = 1;
  while counter <= 5 { // Condición para continuar el bucle
      println!("Contador: {}", counter);
      counter += 1; // Incrementamos el contador
  }

  // **5. Loop infinito con interrupción**
  println!("\n**5. Loop infinito con interrupción:**");
  let mut n = 1;
  loop {
      println!("Valor: {}", n);
      n += 1;
      if n > 5 { // Interrumpimos el bucle cuando se cumple la condición
          break;
      }
  }

  // **6. Skip (continue) y break en for loops**
  println!("\n**6. Skip (continue) y break en for loops:**");
  for i in 1..=5 {
      if i == 2 {
          println!("Saltamos el valor: {}", i);
          continue; // Salta a la siguiente iteración
      }
      if i == 4 {
          println!("Interrumpimos el bucle en el valor: {}", i);
          break; // Sale del bucle
      }
      println!("Iteración actual: {}", i);
  }

  // **7. Propiedad en los bucles (Ownership en for)**
  println!("\n**7. Propiedad en los bucles (Ownership en for):**");
  let vec1 = vec![1, 2, 3, 4, 5];
  for value in vec1.into_iter() { // Transfiere ownership al bucle
      println!("Valor: {}", value);
  }
  // println!("{:?}", vec1); // Error: Ownership de vec1 se movió al bucle

  println!("\nUsando referencias para mantener ownership:");
  let vec2 = vec![10, 20, 30, 40, 50];
  for value in &vec2 { // Borrowing inmutable
      println!("Valor: {}", value);
  }
  println!("El vec2 aún es accesible: {:?}", vec2);

  // **8. Comparación entre for y while**
  println!("\n**8. Comparación entre for y while:**");
  let arr = [100, 200, 300, 400, 500];

  println!("Usando while (acceso por índice):");
  let mut index = 0;
  while index < arr.len() { // Riesgo de desbordar los límites del array
      println!("El valor es: {}", arr[index]);
      index += 1;
  }

  println!("\nUsando for (sin riesgos de límites):");
  for value in arr.iter() { // Itera de forma segura usando un iterador
      println!("El valor es: {}", value);
  }

  // **9. Loop como expresión**
  println!("\n**9. Loop como expresión:**");
  let mut counter = 0;
  let result = loop {
      counter += 1;
      if counter == 10 {
          break counter * 2; // Retorna un valor al interrumpir el bucle
      }
  };
  println!("El resultado del loop es: {}", result);

  println!("\n**Resumen:**");
  println!("- `if` puede actuar como una expresión para retornar valores.");
  println!("- `for` es seguro y eficiente para iterar en rangos y colecciones.");
  println!("- `while` permite más control, pero con riesgos de límites.");
  println!("- `loop` es útil para repeticiones indefinidas y puede actuar como expresión.");
  println!("- Las herramientas de control de flujo hacen que Rust sea expresivo y seguro.");


  println!("**Solución: Control Flow en Rust**");

  // **1. Uso de `if-else` como expresión**
  let number = 7;
  let result = if number % 2 == 0 { "par" } else { "impar" };
  println!("El número {} es {}", number, result);

  // **2. Iterar con un `for` loop**
  println!("Números pares entre 1 y 10:");
  for i in 1..=10 {
      if i % 2 == 0 {
          println!("{}", i);
      }
  }

  // **3. `while` loop**
  println!("\nNúmeros del 5 al 1:");
  let mut count = 5;
  while count > 0 {
      println!("{}", count);
      count -= 1;
  }

  // **4. `loop` infinito con interrupción**
  println!("\nBucle infinito interrumpido:");
  let mut n = 1;
  loop {
      println!("{}", n);
      if n == 5 {
          break;
      }
      n += 1;
  }

  // **5. Uso de `continue` y `break`**
  println!("\nIteración con `continue` y `break`:");
  for i in 1..=10 {
      if i % 3 == 0 {
          println!("Saltamos el múltiplo de 3: {}", i);
          continue;
      }
      if i == 8 {
          println!("Detenemos el bucle en: {}", i);
          break;
      }
      println!("Iteración actual: {}", i);
  }

  // **6. Propiedad en bucles**
  let vec = vec![10, 20, 30, 40, 50];
  println!("\nImprimiendo valores del vector:");
  for &value in &vec {
      println!("{}", value);
  }
  println!("El vector aún es accesible: {:?}", vec);

  // **7. Loop como expresión**
  let result = loop {
      let mut candidate = 51;
      if candidate % 7 == 0 {
          break candidate;
      }
      candidate += 1;
  };
  println!("\nEl primer múltiplo de 7 mayor a 50 es: {}", result);

  println!("\n¡Ejercicio completado correctamente!");
}
