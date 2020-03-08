use super::parser::{Axis, AdvancedFace, FaceElement};
use super::math::{V3, Mat3x3};

#[derive(Debug)]
pub struct Drill {
    pub d: f64,
    pub theta: f64,
    pub slide: f64,
}

#[derive(Debug)]
pub struct Cutter {
    pub y: f64,
}

#[derive(Debug)]
pub struct Proc {
    pub drills: Vec<Drill>,
    pub cutter: Cutter,
    pub center: V3,
    pub size: V3,
}

fn get_align_mat(y_axis: &V3) -> Mat3x3 {
    let theta_y = -(y_axis.x().powi(2) + y_axis.y().powi(2)).sqrt().atan2(y_axis.z());
    let theta_z = -y_axis.y().atan2(y_axis.x());
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
    r_y.prod(&r_z)
}

fn align(mat: &Mat3x3, ax: &Axis) -> Axis {
    Axis {
        p: mat.prod_vec(&ax.p),
        direction: mat.prod_vec(&ax.direction),
        ref_direction: mat.prod_vec(&ax.ref_direction),
    }
}

fn get_size_and_origin(axes: &Vec<Axis>) -> (V3, V3) {
    let mut mins = [1000000.0;3];
    let mut maxs = [0.0;3];
    for ax in axes {
        for i in 0..3 {
            let v = ax.direction.0[i].abs() * ax.p.0[i];
            mins[i] = if mins[i] > v { v } else { mins[i] };
            maxs[i] = if maxs[i] < v { v } else { maxs[i] };
        }
    }
    let size = V3([
        maxs[0] - mins[0],
        maxs[1] - mins[1],
        maxs[2] - mins[2],
    ]);
    let origin = V3([
        (mins[0] + maxs[0]) / 2.,
        (mins[1] + maxs[1]) / 2.,
        0.0,
    ]);
    (size, origin)
}

fn get_y_axis_and_depth(axes: &Vec<&Axis>) -> (V3, f64) {
    let mut dmax = 0.0;
    let mut dv = V3::default();
    for ax in axes {
        let d = ax.direction.dot(&ax.p);
        if d > dmax {
            dmax = d;
            dv = ax.direction.clone();
        }
    }
    (dv, dmax)
}

fn exclude_side_planes<'a>(axes: &'a Vec<&'a Axis>) -> Vec<&'a Axis> {
    let mut cnts = vec![0; axes.len()];
    for i in 0..axes.len() {
        for j in 0..axes.len() {
            if !axes[i].direction.are_independent(&axes[j].direction) {
                cnts[i] += 1;
            }
        }
    }
    let mut r = Vec::new();
    for i in 0..axes.len() {
        if cnts[i] <= 2 {
            r.push(axes[i])
        }
    }
    r
}

fn cylinders_to_drills(orig: &V3, cylinders: &Vec<(f64, Axis)>) -> Vec<Drill> {
    if cylinders.len() > 0 {
        let mut drills = Vec::new();
        for cylinder in cylinders {
            let p = cylinder.1.p.sub(orig);
            let dir = &cylinder.1.direction;
            let theta = dir.y().atan2(dir.x());
            let slide = p.dot(&dir.cross(&V3([0.0, 0.0, 1.0])));
            drills.push(Drill {
                theta,
                d: p.z(),
                slide,
            });
        }
        drills
    }
    else {
        Vec::new()
    }
}

impl Proc {
    pub fn new(faces: &[AdvancedFace]) -> Self {
        let plane_axes =
            faces.iter()
            .filter_map(
                |face| match &face.elem {
                    FaceElement::Plane(ax) => Some(ax),
                    FaceElement::Cylinder(_, _) => None
                })
            .collect();
        let (dv, dmax) = get_y_axis_and_depth(&exclude_side_planes(&plane_axes));
        let r_mat = get_align_mat(&dv);
        let plane_axes =
            plane_axes.iter()
            .map(|ax| align(&r_mat, &ax))
            .collect();
        let (size, origin) = get_size_and_origin(&plane_axes);
        let cylinders =
            faces.iter()
            .filter_map(
                |face| match &face.elem {
                    FaceElement::Cylinder(r, ax) => Some((*r, align(&r_mat, &ax))),
                    _ => None,
                })
            .collect();
        Proc {
            size: size,
            center: origin.clone(),
            cutter: Cutter {
                y: dmax,
            },
            drills: cylinders_to_drills(&origin, &cylinders),
        }
    }
}
