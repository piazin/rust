mod primitives_types;
mod variables;
mod memories;

use primitives_types::primitives_types;
use variables::variables;
use memories::memories;

fn main() {
    variables();
    primitives_types();
    memories();

    //primitives_types::primitives_types();
}
