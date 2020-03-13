use super::analysis::{Proc, Drill};
use std::cmp;
use std::fmt::{Write, Error};

#[derive(Serialize, Deserialize)]
struct AxisOffsetsConfig {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
}

#[derive(Serialize, Deserialize)]
struct EndmillConfig {
    r: f64,
    step: f64,
    offset: f64,
    feed_rate: f64,
}

#[derive(Serialize, Deserialize)]
struct DrillConfig {
    offset: f64,
    feed_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CNCConfig {
    gap_endmill_and_drill: f64,
    feed_rate: f64,
    offsets: AxisOffsetsConfig,
    endmill: EndmillConfig,
    drill: DrillConfig,
    cut: bool,
}

pub struct Move {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
}

impl Move {
    fn nowhere() -> Self {
        Move {
            x: -1.0,
            y: -1.0,
            z: -1.0,
            a: -1.0,
            b: -1.0,
        }
    }
}

pub enum GCode {
    Comment(String),
    G0(Move),
    G1(Move, f64),
    M02,
    M03
}

fn print_modified_axis(buf: &mut String, prefix: &str, before: f64, after: f64) -> Result<(), Error> {
    if (before - after).abs() < 10e-15 {
        Ok(())
    }
    else {
       buf.write_fmt(format_args!("{}{:.3}", prefix, after))
    }
}

fn print_modified_pos(buf: &mut String, before: &Move, after: &Move) -> Result<(), Error> {
    print_modified_axis(buf, "X", before.x, after.x)?;
    print_modified_axis(buf, "Y", before.y, after.y)?;
    print_modified_axis(buf, "Z", before.z, after.z)?;
    print_modified_axis(buf, "A", before.a, after.a)?;
    print_modified_axis(buf, "B", before.b, after.b)?;
    Ok(())
}

fn output(buf: &mut String, cfg: &CNCConfig, gcodes: &[GCode]) -> Result<(), Error> {
    let mut before = &GCode::M02;
    let mut before_pos = &Move::nowhere();
    let mut before_feed_rate = -1.0;
    for gcode in gcodes {
        match gcode {
            GCode::Comment(comment) => {
                buf.write_fmt(format_args!(";{}\n", comment))?;
            },
            GCode::M02 => {
                buf.write_fmt(format_args!("M02\n"))?;
                before = gcode;
            },
            GCode::M03 => {
                buf.write_fmt(format_args!("M03\n"))?;
                before = gcode;
            },
            GCode::G0(m) => {
                match before {
                    GCode::G0(_) => {},
                    _ => {buf.write_fmt(format_args!("G0 "))?;}
                }
                print_modified_pos(buf, before_pos, m)?;
                buf.write_str("\n")?;
                before = gcode;
                before_pos = m;
            },
            GCode::G1(m, feed_rate) => {
                match before {
                    GCode::G1(_, _) => {
                        if *feed_rate != before_feed_rate {
                            buf.write_fmt(format_args!("G1 "))?;
                        }
                    },
                    _ => {
                        buf.write_fmt(format_args!("G1 "))?;
                    }
                }
                print_modified_pos(buf, before_pos, m)?;
                print_modified_axis(buf, "F", before_feed_rate, *feed_rate)?;
                buf.write_str("\n")?;
                before = gcode;
                before_pos = m;
                before_feed_rate = *feed_rate;
            },
        }
    }
    Ok(())
}

fn gcodes_of_drill(cfg: &CNCConfig, drill: &Drill, target_r: f64) -> Vec<GCode> {
    vec![
        GCode::G0(Move {
            x: drill.d,
            y: drill.slide,
            z: target_r + cfg.drill.offset,
            a: drill.theta * 180.0 / std::f64::consts::PI,
            b: target_r + cfg.endmill.offset,
        }),
        GCode::G1(Move {
            x: drill.d,
            y: drill.slide,
            z: 0.0,
            a: drill.theta * 180.0 / std::f64::consts::PI,
            b: target_r + cfg.endmill.offset,
        },
            cfg.drill.feed_rate,
        ),
        GCode::G0(Move {
            x: drill.d,
            y: drill.slide,
            z: target_r + cfg.drill.offset,
            a: drill.theta * 180.0 / std::f64::consts::PI,
            b: target_r + cfg.endmill.offset,
        })
    ]
}

fn gcodes_of_cut(cfg: &CNCConfig, cut_pos: f64, target_r: f64) -> Vec<GCode> {
    let mut gcodes = Vec::new();
    let drill_waiting = target_r + cfg.drill.offset;
    let iter_times = (target_r / cfg.endmill.step / 2.0).ceil() as i32;
    gcodes.push(GCode::G0(Move {
        x: cut_pos,
        y: 0.0,
        z: drill_waiting,
        b: drill_waiting,
        a: 0.0,
    }));
    gcodes.push(GCode::G1(Move {
        x: cut_pos,
        y: 0.0,
        z: drill_waiting,
        b: target_r,
        a: 0.0,
    },
        cfg.feed_rate,
    ));
    for i in 0..iter_times {
        gcodes.push(GCode::G1(Move {
            x: cut_pos,
            y: 0.0,
            z: drill_waiting,
            b: target_r - ((i * 2 + 1) as f64) * cfg.endmill.step,
            a: 360.0,
        },
            cfg.endmill.feed_rate,
        ));
        gcodes.push(GCode::G1(Move {
            x: cut_pos,
            y: 0.0,
            z: drill_waiting,
            b: target_r - ((i * 2 + 2) as f64) * cfg.endmill.step,
            a: 0.0,
        },
            cfg.endmill.feed_rate,
        ));
    }
    gcodes.push(GCode::G1(Move {
        x: cut_pos,
        y: 0.0,
        z: drill_waiting,
        b: drill_waiting,
        a: 0.0,
    },
        cfg.feed_rate,
    ));
    gcodes
}

enum Job<'a> {
    Drill(&'a Drill),
    Cut,
}

pub fn gen_gcode(proc: Proc, cfg: &CNCConfig) -> Result<String, Error> {
    let mut jobs = proc.drills.iter().map(|drill| (drill.d, Job::Drill(drill))).collect::<Vec<(f64, Job)>>();
    if cfg.cut {
        jobs.push((cfg.gap_endmill_and_drill - cfg.endmill.r, Job::Cut));
        jobs.push((cfg.gap_endmill_and_drill + cfg.endmill.r + proc.size.z(), Job::Cut));
    }
    jobs.sort_by(|x, y| if x.0 > y.0 { cmp::Ordering::Greater } else { cmp::Ordering::Less });
    let target_r = ((proc.size.x() / 2.0).powi(2) + (proc.size.y() / 2.0).powi(2)).sqrt();
    let mut gcodes = Vec::new();
    gcodes.push(GCode::Comment("init".to_owned()));
    gcodes.push(GCode::M02);
    for job in jobs {
        match job {
            (_, Job::Drill(drill)) =>
                gcodes.append(&mut gcodes_of_drill(&cfg, &drill, target_r)),
            (p, Job::Cut) =>
                gcodes.append(&mut gcodes_of_cut(cfg, p, target_r)),
        }
    }
    gcodes.push(GCode::M03);
    let mut buf = String::new();
    output(&mut buf, cfg, &gcodes)?;
    Ok(buf)
}
