use std::io::stdin as ioget;

fn main() {

    let decimal:u128 = get_number( false );
 
    //let binary:&str = 
    let binary:String = convert_decimal_to_binary(decimal);

    display_result(decimal, binary);

}

fn display_result(decimal:u128, binary:String){
    println!("Decimal => {}", decimal);
    println!("Binary  => {}", binary);
}

fn convert_decimal_to_binary(number: u128) -> String{

    let mut number_to_divise:u128 = number;
    let mut binary_number:String = String::from("");

    while number_to_divise > 0{

        let rest = number_to_divise % 2;
        number_to_divise = number_to_divise / 2;
        
        match rest {
            0 => binary_number = "0".to_string() + &binary_number,
            1 => binary_number = "1".to_string() + &binary_number,
            _ => binary_number.push_str("") // je vois pas pourquoi car x(u128)%2 vaut toujours 0 ou 1 mais mettons
        }

    }

    if number == 0{
        return "0".to_string();
    }

    return binary_number;
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