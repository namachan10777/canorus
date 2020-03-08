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
}

fn ax_of_face(face: &AdvancedFace) -> Axis {
    match &face.elem {
        FaceElement::Plane(ax) => ax.clone(),
        FaceElement::Cylinder(_, ax) => ax.clone(),
    }
}

fn origin(x_ax: &V3, axes: Vec<&Axis>) -> (f64, f64) {
    for ax in axes {
    }
    (0.0, 0.0)
}

fn get_y_axis_and_depth(axes: Vec<&Axis>) -> (V3, f64) {
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

fn exclude_side_planes(axes: Vec<&Axis>) -> Vec<&Axis> {
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

fn cylinders_to_drills(y_ax: &V3, cylinders: Vec<(&f64, &Axis)>) -> Vec<Drill> {
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
        let cylinders =
            faces.iter()
            .filter_map(
                |face| match &face.elem {
                    FaceElement::Cylinder(r, ax) => Some((r, ax)),
                    _ => None,
                })
            .collect();
        let (dv, dmax) = get_y_axis_and_depth(exclude_side_planes(plane_axes));
        Proc {
            cutter: Cutter {
                y: dmax,
            },
            drills: cylinders_to_drills(&dv, cylinders),
        }
    }
}
