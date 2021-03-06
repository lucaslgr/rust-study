enum Payment {
  Cash(f32),
  Debit(f32),
  Credit(f32, String),
  Crypto(CryptoData),
  Pix { account_key: String, amount: f32 },
}

struct CryptoData {
  amount: f32,
  crypto_name: String,
}

fn main() {
  let payment_method: Payment = Payment::Pix {
    account_key: String::from("932i13-3213912-312321"),
    amount: 32.03,
  };

  match payment_method {
    Payment::Cash(amount) => {
      println!("Cash! {}", amount)
    }
    Payment::Debit(amount) => {
      println!("Debit! {}", amount)
    }
    Payment::Credit(amount, _) => {
      println!("Credit! {}", amount)
    }
    Payment::Crypto(crypto_data) => {
      println!(
        "Crypto! {} of {}",
        crypto_data.amount, crypto_data.crypto_name
      )
    }
    Payment::Pix {
      account_key,
      amount,
    } => println!("Pix! key {} sends {}", account_key, amount),
    _ => println!("Any method"),
  }
}
