use super::parser::{Axis, AdvancedFace, FaceElement};
use super::math::{V3, Mat3x3};

#[derive(Debug)]
pub struct Drill {
    pub d: f64,
    pub theta: f64,
    pub slide: f64,
}

#[derive(Debug)]
pub struct Proc {
    pub drills: Vec<Drill>,
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

// TODO: robustize
fn get_size_and_origin(axes: &(V3, V3, V3), plane_axes: &[Axis]) -> (V3, V3) {
    let mut mins = [1_000_000.0;3];
    let mut maxs = [0.0;3];
    let (x_ax, y_ax, z_ax) = axes;
    for ax in plane_axes {
        let lengthes = [
            ax.p.dot(x_ax),
            ax.p.dot(y_ax),
            ax.p.dot(z_ax),
        ];
        for i in 0..lengthes.len() {
            mins[i] = if lengthes[i] < mins[i] { lengthes[i] } else { mins[i] };
            maxs[i] = if lengthes[i] > maxs[i] { lengthes[i] } else { maxs[i] };
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

struct VecMap<V> {
    inner: Vec<(V3, V)>,
}

impl<V> VecMap<V> {
    fn new() -> Self {
        VecMap {
            inner: Vec::new(),
        }
    }

    fn insert(&mut self, k: V3, v: V) {
        for i in 0..self.inner.len() {
            let (k_ref, _) = &self.inner[i];
            if !k_ref.are_independent(&k) {
                self.inner[i] = (k, v);
                return
            }
        }
        self.inner.push((k, v));
    }

    fn get(&self, k: &V3) -> Option<&V> {
        for i in 0..self.inner.len() {
            let (k_ref, v) = &self.inner[i];
            if !k_ref.are_independent(&k) {
                return Some(&v)
            }
        }
        None
    }

    fn iter(&self) -> ::std::slice::Iter<(V3, V)> {
        self.inner.iter()
    }
}

// 底面のaxisは線形従属なのは2本だけ
// 側面はそれぞれ4本ある
// 押出方向次第ではこの仮定も成り立たない?（そんな事は無い気もする）
// axes -> (x, y, z)
fn get_axes <'a>(axes: &'a [&'a Axis]) -> (V3, V3, V3) {
    let mut map = VecMap::new();
    for ax in axes {
        if let Some(cnt) = map.get(&ax.direction) {
            let new_cnt = *cnt + 1;
            map.insert(ax.direction.clone(), new_cnt);
        }
        else {
            map.insert(ax.direction.clone(), 1);
        }
    }
    let mut x_ax = V3::default();
    let mut y_ax = V3::default();
    let mut z_ax = V3::default();
    for (v, cnt) in map.iter() {
        if *cnt == 2 {
            z_ax = v.normalize();
        }
        else if x_ax == V3::default() {
            x_ax = v.normalize();
        }
        else {
            y_ax = v.normalize();
        }
    }
    (x_ax, y_ax, z_ax)
}

fn cylinders_to_drills(orig: &V3, cylinders: &[(f64, Axis)]) -> Vec<Drill> {
    if !cylinders.is_empty() {
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
        let plane_axes=
            faces.iter()
            .filter_map(
                |face| match &face.elem {
                    FaceElement::Plane(ax) => Some(ax),
                    FaceElement::Cylinder(_, _) => None
                })
            .collect::<Vec<&Axis>>();
        let (ax_x, ax_y, ax_z) = get_axes(plane_axes.as_slice());
        let r_mat = get_align_mat(&ax_z);
        let ax_x = r_mat.prod_vec(&ax_x);
        let ax_y = r_mat.prod_vec(&ax_y);
        let ax_z = r_mat.prod_vec(&ax_z);
        let axes = (ax_x, ax_y, ax_z);
        let plane_axes =
            plane_axes.iter()
            .map(|ax| align(&r_mat, &ax))
            .collect::<Vec<Axis>>();
        let (size, origin) = get_size_and_origin(&axes, plane_axes.as_slice());
        let cylinders : Vec<(f64, Axis)> =
            faces.iter()
            .filter_map(
                |face| match &face.elem {
                    FaceElement::Cylinder(r, ax) => Some((*r, align(&r_mat, &ax))),
                    _ => None,
                })
            .collect();
        Proc {
            size,
            center: origin.clone(),
            drills: cylinders_to_drills(&origin, cylinders.as_slice()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vecmap() {
        let mut map = VecMap::new();
        map.insert(V3([1.0, 0.0, 0.0]), 1);
        map.insert(V3([0.0, 0.0, 1.0]), 2);
        assert_eq!(map.get(&V3([3.0, 0.0, 0.0])), Some(&1));
        assert_eq!(map.get(&V3([1.0, 0.0, 1.0])), None);
    }
}
