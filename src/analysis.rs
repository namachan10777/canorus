use super::parser::{Axis, AdvancedFace, FaceElement, V3};

type L3x3 = [[f64; 3]; 3];

fn inv(a: L3x3) -> L3x3 {
    for i in 0..3 {
        for j in 0..3 {
        }
    }
    a
}

fn prod_mat_mat(a: &L3x3, b: &L3x3) -> L3x3 {
    let y = L3x3::default();
    for i in 0..3 {
        for j in 0..3 {
        }
    }
    y
}

fn prod_mat_vec(a: &L3x3, b: V3) -> V3 {
    b
}

pub fn align(dir: &V3, ref_dir: &V3, axis: Axis) -> Axis {
    let (x1, y1, z1) = dir;
    let (x2, y2, z2) = ref_dir;
    let y = [
        [*x1, *x2, 1.0],
        [*y1, *y2, 0.0],
        [*z1, *z2, 0.0],
    ];
    let (x3, y3, z3) = axis.direction;
    let (x4, y4, z4) = axis.ref_direction;
    let x = [
        [x3, x4, 1.0],
        [y3, y4, 0.0],
        [z3, z4, 0.0],
    ];
    let a = prod_mat_mat(&y, &inv(x));
    Axis {
        p: prod_mat_vec(&a, axis.p),
        direction: prod_mat_vec(&a, axis.direction),
        ref_direction: prod_mat_vec(&a, axis.ref_direction),
    }
}
