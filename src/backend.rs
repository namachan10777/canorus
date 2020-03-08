use super::analysis::Proc;
use std::cmp;

const DRILL_PULLING : f64 = 30.0;
const ENDMILL_R : f64 = 3.0;
const GAP_ENDMILL_AND_DRILL : f64 = 40.0;

pub fn gen_gcode(mut proc: Proc) -> String {
    proc.drills.sort_by(|x, y| if x.d > y.d { cmp::Ordering::Greater } else { cmp::Ordering::Less });
    String::new()
}
