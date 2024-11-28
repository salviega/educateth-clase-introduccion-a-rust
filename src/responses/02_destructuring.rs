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

    println!("**Solución: Desestructuración en Rust**");

    // **1. Tuplas**
    let libro = ("El Principito", 1943, 9.99); // Tupla con nombre, año y precio
    let (nombre, _, precio) = libro;
    println!("Nombre del libro: {}, Precio: ${}", nombre, precio);

    // **2. Arreglos**
    let numeros = [10, 20, 30, 40, 50];
    let [primero, segundo, .., ultimo] = numeros;
    println!("Primer número: {}, Segundo número: {}, Último número: {}", primero, segundo, ultimo);

    // **3. Structs**
    struct Producto {
        nombre: String,
        precio: f64,
        stock: u32,
    }

    let producto = Producto {
        nombre: "Teclado".to_string(),
        precio: 49.99,
        stock: 15,
    };

    let Producto { nombre, stock, .. } = producto;
    println!("Producto: {}, Stock disponible: {}", nombre, stock);

    // **4. Combinación de tipos**
    let estructura_compuesta = (
        (5, "Rust"),
        [1, 2, 3],
        Producto {
            nombre: "Monitor".to_string(),
            precio: 199.99,
            stock: 10,
        },
    );

    let ((numero, lenguaje), [primero, ..], Producto { nombre, precio, .. }) = estructura_compuesta;
    println!("Número: {}, Lenguaje: {}, Primer elemento del arreglo: {}, Producto: {}, Precio: ${}", 
             numero, lenguaje, primero, nombre, precio);

    println!("¡Ejercicio completado correctamente!");
}
