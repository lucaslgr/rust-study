enum Payment {
  Cash(f32),
  Debit(f32),
  Credit(f32, String),
  Crypto(CryptoData),
}

struct CryptoData {
  amount: f32,
  crypto_name: String,
}

fn main() {
  // let payment_method: Payment = Payment::Debit(100.0);
  // let payment_method: Payment = Payment::Credit(100.0, String::from("6 installments"));
  let payment_method: Payment = Payment::Crypto(CryptoData {
    amount: 0.1,
    crypto_name: String::from("bitcoin"),
  });

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
    Payment::Crypto(crypto_data) => {
      println!(
        "Crypto! {} of {}",
        crypto_data.amount, crypto_data.crypto_name
      )
    }
    _ => println!("Any method"),
  }
}
