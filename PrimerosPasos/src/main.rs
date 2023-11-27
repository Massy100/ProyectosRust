fn main() {
    /* Esto es un comentario*/

    // Variables
    let mut my_string: &str = "Esto es una cadena de texto";
    my_string = "Aqui cambio el valor de la cadena de texto";
    println!("Variable: {my_string}");

    // my_string = 6; Error
    let my_string2: String = String:: from("Esta es otra cadena de texto");
    println!("{my_string2}");

    let mut my_int: i32 = 7;
    my_int = my_int + 4;
    println!("{}", my_int - 1);
    println!("{my_int}");

    println!("Este es el valor de la variable my_int: {}", my_int);

    let my_int64: i64 = 7;
    println!("{my_int64}");

    let my_float: f64 = 6.5;
    println!("{my_float}");
    //my_float = my_float + my_int; // Error

    let mut my_float2: f32 = 6.5;
    println!("{my_float2}");

    let mut my_bool = false;
    my_bool = true;
    println!("{my_bool}");

    // Constantes
    const MY_CONST: &str = "Mi propiedad constante";
    println!("{MY_CONST}");
    

}
