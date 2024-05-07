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
mod logger;
mod loops;
mod is_palindrome;
*/

/*
use is_palindrome::Palindrome;
use memories::memories;
use primitives_types::primitives_types;
use string::string;
use variables::variables;
use calculations::calculations;
use functions::functions;
use ownership_borrowing::ownership;
use enums::enums;
use indicatif::indicatif;
use loops::loops;
use logger::logger;
*/
mod pointer;
use pointer::Pointer;

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

    // logger();
    // loops();

    // custom traits String
    // let s = String::from("civic");
    // s.is_palindrome();

    //primitives_types::primitives_types();

    // raw pointer
    let my_string = String::from("Hamu");
    println!("{}", my_string.ptr()); // print pointer
}
