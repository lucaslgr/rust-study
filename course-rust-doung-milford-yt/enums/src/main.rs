enum Payment {
    Cash,
    CreditCard,
    DebitCard,
}

fn main() {
    let some_payment_method = Payment::Cash;

    match some_payment_method {
        Payment::Cash => println!("Paying with cash..."),
        Payment::CreditCard => println!("Paying with cash..."),
        Payment::DebitCard => println!("Paying with cash..."),
        _ => {
            println!("Nothing was chose...")
        }
    }
}
