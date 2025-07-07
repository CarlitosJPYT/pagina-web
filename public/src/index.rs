fn main(){
    let edad = 24;
    if edad <= 5{println!("Eres un bebé.");}
    else if edad >= 6 && edad <= 12{println!("Eres un niño.");}
    else if edad >= 13 && edad <= 17{println!("Eres un adolescente.");}
    else if edad >= 18 && edad <= 49{println!("Eres un adulto.");}
    else{println!("Eres un adulto mayor.")}
}