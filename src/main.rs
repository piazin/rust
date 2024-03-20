mod memories;
mod primitives_types;
mod string;
mod variables;
mod calculations;

use memories::memories;
use primitives_types::primitives_types;
use string::string;
use variables::variables;
use calculations::calculations;

fn main() {
    variables();
    primitives_types();
    memories();
    string();
    calculations();

    //primitives_types::primitives_types();
}
