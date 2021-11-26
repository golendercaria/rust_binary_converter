use std::io::stdin as ioget;

fn main() {

    let n = get_number( false );
    println!("Votre nombre est {}",n);

}

// Fonction qui va demander un nombre à l'utilisateur
fn get_number( is_error : bool ) -> u128{

    if !is_error {
        println!("Insert your number:");
    }else{
        println!("How, your number is wrong, try again:");
    }

    let mut text = String::new();

    ioget().read_line(&mut text).expect("Fail to read text");

    let number:u128 = match text.trim().parse(){
        Ok(n) => n,
        Err(_) => {
            return get_number(true);
        }
    };

    return number;
}