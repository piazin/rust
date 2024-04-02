/*
mod memories;
mod primitives_types;
mod string;
mod variables;
mod calculations;
mod functions;
mod ownership_borrowing;
mod enums;
mod indicatif;
*/

mod logger;
mod test;

/*
use memories::memories;
use primitives_types::primitives_types;
use string::string;
use variables::variables;
use calculations::calculations;
use functions::functions;
use ownership_borrowing::ownership;
use enums::enums;
use indicatif::indicatif;
*/
use logger::logger;
use test::answer;

fn main() {
    //variables();
    //primitives_types();
    //memories();
    //string();
    // calculations();
    //functions();
    //ownership();
    // enums();
    // indicatif();

    logger();
    answer(25, 25);

    //primitives_types::primitives_types();
}
