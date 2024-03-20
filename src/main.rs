mod memories;
mod primitives_types;
mod string;
mod variables;
// mod calculations;
mod functions;
mod ownership_borrowing;

use memories::memories;
use primitives_types::primitives_types;
use string::string;
use variables::variables;
// use calculations::calculations;
use functions::functions;
use ownership_borrowing::ownership;

fn main() {
    variables();
    primitives_types();
    memories();
    string();
    // calculations();
    functions();
    ownership();

    //primitives_types::primitives_types();
}
