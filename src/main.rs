mod primitives_types;
mod variables;
mod memories;
mod string;

use primitives_types::primitives_types;
use variables::variables;
use memories::memories;
use string::string;

fn main() {
    variables();
    primitives_types();
    memories();
    string();

    //primitives_types::primitives_types();
}
