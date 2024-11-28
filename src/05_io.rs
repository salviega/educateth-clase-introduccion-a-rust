// Importamos el módulo `io` de la biblioteca estándar para manejar la entrada/salida
use std::io;

fn main() {
    println!("**Escribir y leer en la consola con Rust**");

    // **1. Escribir en la consola**
    println!("\n**1. Escribir en la consola:**");
    println!("Podemos usar la macro `println!` para imprimir texto con un salto de línea.");
    println!("Ejemplo: Hola, este es un mensaje de prueba.");
    print!("Podemos usar la macro `print!` para imprimir texto sin salto de línea. ");
    println!("¡Así continúa en la misma línea!\n");

    // **2. Leer desde la terminal**
    println!("**2. Leer desde la terminal:**");
    
    // Creamos una variable mutable para almacenar la entrada del usuario
    let mut input: String = String::new();
    println!("Escribe tu nombre:");

    // Leemos la entrada del usuario desde la entrada estándar (`stdin`)
    io::stdin()
        .read_line(&mut input) // Leemos y almacenamos en `input`
        .expect("Error al leer la entrada"); // Manejamos posibles errores

    // **3. Usar los datos leídos**
    println!("\n**3. Usar los datos leídos:**");
    println!("Hola, {}! Gracias por participar.", input.trim());

    // **4. Convertir entrada a otro tipo**
    println!("\n**4. Convertir entrada a otro tipo:**");
    println!("Escribe un número:");
    let mut number_input = String::new();
    io::stdin()
        .read_line(&mut number_input)
        .expect("Error al leer la entrada");

    // Convertimos la entrada de tipo String a i32
    let number: i32 = number_input.trim().parse().expect("Por favor, ingresa un número válido.");
    println!("El número que ingresaste es: {}", number);

    // **5. Ejemplo final: Entrada y procesamiento**
    println!("\n**5. Ejemplo final:**");
    println!("Escribe un número para calcular su cuadrado:");
    let mut num_input = String::new();
    io::stdin()
        .read_line(&mut num_input)
        .expect("Error al leer la entrada");

    // Convertimos a i32 y calculamos el cuadrado
    let num: i32 = num_input.trim().parse().expect("Por favor, ingresa un número válido.");
    println!("El cuadrado de {} es {}", num, num * num);


    println!("**Ejercicio práctico: Entrada y salida en Rust**");

    // **1. Escribir mensajes en la consola**
    println!("1. Escribe un mensaje que salude al usuario e indique el propósito del programa.\n");

    // **2. Leer una cadena de texto**
    println!("2. Pide al usuario que escriba el nombre de su ciudad y almacena el valor ingresado en una variable.\n");

    // **3. Convertir entrada a un número**
    println!("3. Solicita al usuario que ingrese su edad. Convierte el valor ingresado a un número entero.\n");

    // **4. Procesar datos ingresados**
    println!("4. Calcula el número de días que el usuario ha vivido basándote en su edad (aproximadamente 365 días por año).\n");

    // **5. Mostrar un resumen**
    println!("5. Muestra un resumen que incluya la ciudad, la edad y los días vividos del usuario.\n");

    println!("¡Completa el ejercicio para practicar la entrada y salida en Rust!");

}
