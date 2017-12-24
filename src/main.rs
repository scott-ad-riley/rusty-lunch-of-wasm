#[macro_use]
extern crate stdweb;
extern crate rand;

use rand::Rng;

static FOOD_PLACES: [&str; 16] = [
    "Bar Burrito",
    // "Bento Ya",
    "Dough",
    "Herbivore Kitchen",
    "Hula",
    "Let us eat",
    // "Maki and Ramen",
    "Mama's",
    "Pizza Express",
    "Pumpkin Brown",
    "Redbox",
    "Söderberg",
    "Taquito",
    "The Red Squirrel",
    "Ting Thai Caravan",
    "Wagamama",
    "Wildman Wood",
    "Zizzi's",
];

fn choose() -> String {
    let mut rng = rand::thread_rng();
    let choice = rng.choose(&FOOD_PLACES).unwrap();
    String::from("blah")
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.choose = @{choose};
    }
}
