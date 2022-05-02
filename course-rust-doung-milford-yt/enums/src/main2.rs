enum Payment {
  Cash(f32),
  Debit(f32),
  Credit(f32, String),
}

fn main() {
  // let payment_method: Payment = Payment::Debit(100.0);
  let payment_method: Payment = Payment::Credit(100.0, String::from("6 installments"));

  match payment_method {
    Payment::Cash(amount) => {
      println!("Cash! {}", amount)
    }
    Payment::Debit(amount) => {
      println!("Debit! {}", amount)
    }
    Payment::Credit(amount, num_installments) => {
      println!("Credit! {} by {} ", amount, num_installments)
    }
    _ => println!("Any method"),
  }
}
