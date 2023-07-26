use pipetrait::PipeTrait;
use std::sync::{Arc, Mutex};
fn main() {
    // Look, as you see, what the power is it
    // So it is a toy
    // Idea from kotin
    let _ = 1
        .pipe(Mutex::new)
        .pipe(Arc::new)
        .pipe(Mutex::new)
        .pipe(Arc::new);
}
