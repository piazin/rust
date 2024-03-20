mod memories;
mod primitives_types;
mod string;
mod variables;
// mod calculations;
mod functions;

use memories::memories;
use primitives_types::primitives_types;
use string::string;
use variables::variables;
// use calculations::calculations;
use functions::functions;

fn main() {
    variables();
    primitives_types();
    memories();
    string();
    // calculations();
    functions();

    //primitives_types::primitives_types();
}
