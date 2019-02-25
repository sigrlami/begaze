use std::io;

use types::*;

////////////////////////////////////////////////////////////////////////////////

trait Listener {
    fn initSession(&self) -> ;
    fn killSession(&self) -> ;
    fn listen(&self)
}


// we need to have ability to create different types of listeners
// regular, tree, channel
