use super::parser::{Axis, AdvancedFace, FaceElement};
use super::math::{V3, Mat3x3};

#[derive(Debug)]
pub struct Drill {
    pub y: f64,
    pub theta: f64,
}

#[derive(Debug)]
pub struct Cutter {
    pub y: f64,
}

#[derive(Debug)]
pub struct Proc {
    drills: Vec<Drill>,
    cutter: Cutter,
    center: V3,
    size: V3,
}

fn ax_of_face(face: &AdvancedFace) -> Axis {
    match &face.elem {
        FaceElement::Plane(ax) => ax.clone(),
        FaceElement::Cylinder(_, ax) => ax.clone(),
    }
}

fn get_size_and_origin(axes: &Vec<&Axis>) -> (V3, V3) {
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
        (mins[2] + maxs[2]) / 2.,
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

fn cylinders_to_drills(y_ax: &V3, cylinders: &Vec<(&f64, &Axis)>) -> Vec<Drill> {
    let mut drills = Vec::new();
    for cylinder in cylinders {
    }
    drills
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
        let (size, origin) = get_size_and_origin(&plane_axes);
        let cylinders =
            faces.iter()
            .filter_map(
                |face| match &face.elem {
                    FaceElement::Cylinder(r, ax) => Some((r, ax)),
                    _ => None,
                })
            .collect();
        let (dv, dmax) = get_y_axis_and_depth(&exclude_side_planes(&plane_axes));
        Proc {
            size: size,
            center: origin,
            cutter: Cutter {
                y: dmax,
            },
            drills: cylinders_to_drills(&dv, &cylinders),
        }
    }
}
