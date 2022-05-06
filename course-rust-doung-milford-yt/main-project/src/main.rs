mod geo_map;

fn main() {
    let my_favourite_place = geo_map::get_hawai_location();
    println!(
        "My favorite place is: lat={} and long={}",
        my_favourite_place.latitude, my_favourite_place.longitude
    );
}
