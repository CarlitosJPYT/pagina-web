fn main(){
    let temperatura = 25;
    if temperatura < 25{celsius(20);}
    else if temperatura == 25{
        let celsius = (temperatura - 32) * 5/9;
        println!("La temperatura es de {celsius}");
    }
    else {fahrenheit(50);}
}
fn celsius(temperatura: i32){
    let resultado = temperatura * 5/9 + 32;
    println!("La temperatura es de {resultado}");
}
fn fahrenheit(temperatura: i32){
    let operacion = temperatura - 343;
    println!("La temperatura es de {operacion}");
}