use std::io::stdin as ioget;

fn main() {

    let (original_number, number_to_vector) = get_number( false );

    let binary:String = convert_decimal_to_binary(number_to_vector);

    display_result(original_number, binary);

}

fn display_result(original_number:String, binary:String){
    println!("Decimal => {}", original_number);
    println!("Binary  => {}", binary);
}

// vector reducer => divider by 2
// ex: 256 => 128 => 64 => ...
fn reduce_vector(number_to_vector:Vec<u8>) -> Vec<u8>{

    let mut new_number_to_vector:Vec<u8> = Vec::new();
    let mut rest:u8 = 0;

    for i in number_to_vector{

        new_number_to_vector.push(i/2 + rest);

        if i == 1 || i%2 == 1{
            rest = 5;
        }else{
            rest = 0;
        }

    }

    // remove first 0
    if new_number_to_vector[0] == 0{
        new_number_to_vector.remove(0);
    }

    return new_number_to_vector;
}

/**
* 
*   Principe
*   
*   2 5 6
*   6%2 => 0    STEP1 => MOD2 is binary value
*   / / /
*   1 2 3       STEP2 => divive each number by 2
*       +5      if rest distribute 5 to next element
*
*   1 2 8       REPEAT
*
*/
fn convert_decimal_to_binary(mut number_to_vector: Vec<u8>) -> String{

    let mut binary_number:String = String::from("");
    let mut len_of_number = number_to_vector.len();

    while len_of_number > 0{

        // calcul binary value
        let last_number = number_to_vector[ len_of_number - 1 ];

        match last_number % 2 {
            0 => binary_number = "0".to_string() + &binary_number,
            1 => binary_number = "1".to_string() + &binary_number,
            _ => binary_number.push_str("")
        }

        // reduce vector
        number_to_vector = reduce_vector(number_to_vector);
        
        len_of_number = number_to_vector.len();

    }

    return binary_number;

}

// Fonction qui va demander un nombre à l'utilisateur et retourner un tupple
fn get_number( is_error : bool ) -> (String, Vec<u8>){

    if !is_error {
        println!("Insert your number:");
    }else{
        println!("How, your number is wrong, try again:");
    }

    let mut text                        = String::new();
    let mut number_to_vector:Vec<u8>    = Vec::new();

    ioget().read_line(&mut text).expect("Fail to read text");

    for c in text.trim().chars(){
        number_to_vector.push(
            match c.to_string().parse(){
                Ok(n) => n,
                Err(_) => {
                    return get_number(true);
                }
            }
        );
    }

    // check if vector is empty
    if number_to_vector.len() == 0{
        get_number(true);
    }

    return (text.trim().to_string(), number_to_vector);
}