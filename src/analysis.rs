use super::parser::{Axis, AdvancedFace, FaceElement};
use super::math::{V3, Mat3x3};

fn align(ref_dir: &V3, axis: Axis) -> Axis {
    let dir1 = ref_dir;
    let dir2 = &axis.ref_direction;
    let theta_y =
        (dir1.x().powi(2) + dir1.y().powi(2)).sqrt().atan2(dir1.z())
        - (dir2.x().powi(2) + dir2.y().powi(2)).sqrt().atan2(dir2.z());
    let theta_z =
        dir1.y().atan2(dir1.x())
        - dir2.y().atan2(dir2.x());
    let r_y = Mat3x3([
        V3([ theta_y.cos(), 0.0, theta_y.sin()]),
        V3([           0.0, 1.0,           0.0]),
        V3([-theta_y.sin(), 0.0, theta_y.cos()]),
    ]);
    let r_z = Mat3x3([
        V3([theta_z.cos(), -theta_z.sin(), 0.0]),
        V3([theta_z.sin(),  theta_z.cos(), 0.0]),
        V3([          0.0,            0.0, 1.0]),
    ]);
    let r = r_y.prod(&r_z);
    println!("{:?}", r.prod_vec(&axis.ref_direction));
    Axis {
                    p: r.prod_vec(&axis.p),
            direction: r.prod_vec(&axis.direction),
        ref_direction: r.prod_vec(&axis.ref_direction),
    }
}

pub fn align_face(face: AdvancedFace) -> AdvancedFace {
    AdvancedFace {
        flag: face.flag,
        elem: match face.elem {
            FaceElement::Plane(axis) => FaceElement::Plane(align(&V3([1.0, 0.0, 0.0]), axis)),
            FaceElement::Cylinder(r, axis) => FaceElement::Cylinder(r, align(&V3([1.0, 0.0, 0.0]), axis)),
        },
    }
}
