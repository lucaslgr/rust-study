pub struct Localization {
  pub latitude: f64,
  pub longitude: f64,
}

pub fn get_hawai_location() -> Localization {
  Localization {
    latitude: 32.3,
    longitude: 312.3,
  }
}
