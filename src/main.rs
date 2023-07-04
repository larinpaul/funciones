fn mi_funcion() {

    println!("¡Hola de nuevo amigos!");

}

fn main() {

    println!("¡Hola amigos!");

    mi_funcion();

    valor_variable(42, 24);

    // Declaraciones y expresiones

    // Los cuerpos de las funciones se componen de una serie de
    // declaraciones que, opcionalmente, terminan en una expresión.

    // Las declaraciones son instrucciones que realizan alguna acción y no
    // devuelven un valor, mientras que las expresiones se evalúan y
    // devuelven el valor resultante.

    let ii = 26;

    let jj = {
        let k = ii + 1;

        k + 1
    };

    println!("El valor de jj es: {}", jj); // 28

    // Funciones que devuelven valores

    // Las funciones pueden devlover valores al código que las llama.
    // Para ello, debemos declarar el tipo de datos que devuelven
    // despues de la flecha ->.

    // En Rust, el valor de retorno de la función es sinónimo del valor de la
    // expresión final en el bloque del cuerpo de una función. Puede regresar
    // antes de una función utilizando la palabra clave return y especificando
    // un valor, pero la mayoría de las functiones devuelven la última expresión
    // implícitamente.

    let iii = trece();

    println!("El valor de iii es: {}", iii); // 13

    fn trece() -> i8 {
        return 13 
    }

}

// Parámetros de Funciones

// Las funciones también se pueden definir para que tengan parámetros,
// que son variables especiales que van a tomar valor cuando se invoque
// la dicha functión.

fn valor_variable(i: i32, j: i32) {

    println!("El valor del parametro es : {}", i);
    println!("El valor del segundo parametro es : {}", j);

}





