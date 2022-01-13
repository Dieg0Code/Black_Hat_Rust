use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};


const SHA1_HEX_STRING_LENGTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect(); // recolecta los argumentos en un vector


    // corroboramos que el vector tenga los argumentos necesarios
    if args.len() != 3 {
        print!("Uso:");
        print!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim(); // obtenemos el hash a desencriptar
    // si la longitud del hash no es de 40 caracteres, no es un hash valido
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return  Err("sha1 hash invalido".into());
    }

    // Ya que toma demasiado tiempo probar todas las posibles combinaciones de letras, numeros y
    // caracteres especiales, necesitamos reducir el numero de hashes SHA-1 generados. Para ello 
    // utilizamos un tipo especial de diccionario, conocido como Wordlist, el cual contiene las 
    // contraseñas mas comunes encontradas en websites.
    let word_list_file = File::open(&args[1])?; // abrimos el archivo con la wordlist
    let reader = BufReader::new(&word_list_file); // creamos un lector de archivos

    for line in reader.lines() { // iteramos sobre cada linea del archivo
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack ==
            &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
                print!("Contraseña encontrada: {}", &common_password);
                return Ok(());
            }
    }

    print!("Contraseña no encontrada en la wordlist :(");

    Ok(())
}
