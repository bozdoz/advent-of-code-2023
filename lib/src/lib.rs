use std::env;

pub fn get_part() -> (bool, bool) {
    let args = env::args().skip(1);

    let mut hasone = false;
    let mut hastwo = false;

    for arg in args {
        if arg.contains(&String::from("one")) {
            hasone = true;
        }
        if arg.contains(&String::from("two")) {
            hastwo = true;
        }   
    }

    if !hasone && !hastwo {
        // run them both by default
        hasone = true;
        hastwo = true;
    }
    
    (hasone, hastwo)
}

// TODO: do we need this?
#[allow(dead_code)]
fn lerp(x: usize, x1: usize, x2: usize, y1: usize, y2: usize) -> usize {
    return y1 + ((y2 - y1) / (x2 - x1)) * (x - x1);
}
