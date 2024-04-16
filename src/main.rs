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
<<<<<<< HEAD
mod loops;
=======
mod test;
>>>>>>> 988058e0485ec6c44a3c1ca9d6b761e0597aedc6

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
<<<<<<< HEAD
use loops::loops;
=======
use test::answer;
>>>>>>> 988058e0485ec6c44a3c1ca9d6b761e0597aedc6

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
<<<<<<< HEAD
    loops();
=======
    answer(25, 25);
>>>>>>> 988058e0485ec6c44a3c1ca9d6b761e0597aedc6

    //primitives_types::primitives_types();
}
