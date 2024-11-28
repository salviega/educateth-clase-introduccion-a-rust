fn main() {
    println!("**1. ¿Qué es la desestructuración en Rust?**");
    println!("La desestructuración es el proceso de dividir valores de tipos compuestos (como tuplas, arreglos, structs, enums, etc.) en variables individuales. Permite extraer valores específicos de manera concisa y eficiente.\n");

    // **2. Desestructuración de tuplas**
    println!("**2. Desestructuración de tuplas:**");
    let (x, y, z) = (1, 2, 3); // Desestructuración completa
    println!("Valores extraídos de la tupla: x = {}, y = {}, z = {}", x, y, z);

    let (a, b, _) = (4, 5, 6); // Ignorando un elemento con _
    println!("Valores extraídos ignorando el tercer elemento: a = {}, b = {}\n", a, b);

    // **3. Desestructuración de arreglos**
    println!("**3. Desestructuración de arreglos:**");
    let [c, d, .., e] = [10, 20, 30, 40, 50]; // Ignorando elementos intermedios con `..`
    println!("Valores extraídos del arreglo: c = {}, d = {}, e = {}\n", c, d, e);

    // **4. Desestructuración de structs**
    println!("**4. Desestructuración de structs:**");
    struct Ferris {
        x: i32,
        y: String,
    }

    let ferris = Ferris { x: 42, y: "Rustacean".to_string() };
    let Ferris { x, y } = ferris; // Desestructurando los campos del struct
    println!("Valores extraídos del struct Ferris: x = {}, y = {}\n", x, y);

    // **5. Desestructuración parcial**
    println!("**5. Desestructuración parcial:**");
    let ferris = Ferris { x: 7, y: "Rust".to_string() };
    let Ferris { x, .. } = ferris; // Ignorando el campo `y` con `..`
    println!("Valor extraído parcialmente del struct Ferris: x = {}\n", x);

    // **6. Usos combinados**
    println!("**6. Combinación de desestructuraciones:**");
    let data = ((1, 2), [3, 4, 5], Ferris { x: 6, y: "Rust".to_string() });
    let ((a, b), [c, ..], Ferris { x, y }) = data;
    println!("Valores extraídos de combinaciones de tipos:");
    println!("a = {}, b = {}, c = {}, x = {}, y = {}\n", a, b, c, x, y);

    // **7. Ventajas de la desestructuración**
    println!("**7. Ventajas de la desestructuración:**");
    println!("- Código más legible y conciso.");
    println!("- Control preciso sobre los datos que deseas extraer.");
    println!("- Ignorar datos irrelevantes fácilmente con `_` o `..`.\n");

    println!("¡Todo listo para usar desestructuración en Rust de manera eficiente!");

    /* //////////////////////////////// */
    
    println!("**Ejercicio práctico: Desestructuración en Rust**");

    // **1. Tuplas**
    println!("1. Crea una tupla que contenga el nombre de un libro, el año de publicación y el precio.");
    println!("   Desestructura la tupla para obtener el nombre y el precio, ignorando el año.\n");

    // **2. Arreglos**
    println!("2. Declara un arreglo con cinco números enteros.");
    println!("   Usa desestructuración para obtener el primer, segundo y último valor del arreglo.\n");

    // **3. Structs**
    println!("3. Define un struct llamado `Producto` con los campos `nombre`, `precio` y `stock`.");
    println!("   Crea una instancia del struct y desestructúrala para extraer solo el `nombre` y el `stock`.\n");

    // **4. Combinación de tipos**
    println!("4. Crea una estructura que combine una tupla, un arreglo y un struct.");
    println!("   Desestructura esa estructura para acceder a los valores internos.\n");

    println!("¡Resuelve los ejercicios e implementa el código para entender mejor la desestructuración!");
}
