fn main() {

    //iterator for_in de forma inclusiva

    for numero in 1..10 {

        println!("el valor del numero es (inclusiva): {:#?}", numero);
        

    }

    //iterator for_in de forma exclusiva

    for numero2 in 1..=10 {
        println!("\t el valor de numero es (exclusiva): {:#?}", numero2);
    }


}

