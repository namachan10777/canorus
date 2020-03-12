use super::analysis::{Proc, Drill};
use std::cmp;
use std::fmt::{Write, Error};
use serde::{Deserialize, Serialize};

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
}

pub struct Move {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
    feed_rate: f64,
}

pub enum GCode {
    Comment(String),
    G0(Move),
    G1(Move),
    M02,
    M03
}

fn output(buf: &mut String, cfg: &CNCConfig, gcodes: &[GCode]) -> Result<(), Error> {
    let mut before = &GCode::M02;
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
                    _ => {buf.write_fmt(format_args!("G0\n"))?;}
                }
                buf.write_fmt(format_args!("X{:.3}Y{:.3}Z{:.3}A{:.3}B{:.3}F{:.3}\n",
                        m.x + cfg.offsets.x,
                        m.y + cfg.offsets.y,
                        m.z + cfg.offsets.z,
                        m.a + cfg.offsets.a,
                        m.b + cfg.offsets.b,
                        m.feed_rate
                ))?;
                before = gcode;
            },
            GCode::G1(m) => {
                match before {
                    GCode::G1(_) => {},
                    _ => {buf.write_fmt(format_args!("G0\n"))?;}
                }
                buf.write_fmt(format_args!("X{:.3}Y{:.3}Z{:.3}A{:.3}B{:.3}F{:.3}\n",
                        m.x + cfg.offsets.x,
                        m.y + cfg.offsets.y,
                        m.z + cfg.offsets.z,
                        m.a + cfg.offsets.a,
                        m.b + cfg.offsets.b,
                        m.feed_rate
                ))?;
                before = gcode;
            },
        }
    }
    Ok(())
}

fn gcodes_of_drill(cfg: &CNCConfig, drill: &Drill, target_r: f64) -> Vec<GCode> {
    vec![
        GCode::G1(Move {
            x: drill.d,
            y: drill.slide,
            z: target_r + cfg.drill.offset,
            a: drill.theta * 360.0 / std::f64::consts::PI,
            b: target_r + cfg.endmill.offset,
            feed_rate: cfg.feed_rate,
        }),
        GCode::G1(Move {
            x: drill.d,
            y: drill.slide,
            z: 0.0,
            a: drill.theta * 360.0 / std::f64::consts::PI,
            b: target_r + cfg.endmill.offset,
            feed_rate: cfg.drill.feed_rate,
        }),
        GCode::G1(Move {
            x: drill.d,
            y: drill.slide,
            z: target_r + cfg.drill.offset,
            a: drill.theta * 360.0 / std::f64::consts::PI,
            b: target_r + cfg.endmill.offset,
            feed_rate: cfg.feed_rate,
        })
    ]
}

fn gcodes_of_cut(cfg: &CNCConfig, cut_pos: f64, target_r: f64) -> Vec<GCode> {
    let mut gcodes = Vec::new();
    let drill_waiting = target_r + cfg.drill.offset;
    let iter_times = (target_r / cfg.endmill.step / 2.0).ceil() as i32;
    gcodes.push(GCode::G1(Move {
        x: cut_pos,
        y: 0.0,
        z: drill_waiting,
        b: drill_waiting,
        a: 0.0,
        feed_rate: cfg.feed_rate,
    }));
    gcodes.push(GCode::G1(Move {
        x: cut_pos,
        y: 0.0,
        z: drill_waiting,
        b: target_r,
        a: 0.0,
        feed_rate: cfg.feed_rate,
    }));
    for i in 0..iter_times {
        gcodes.push(GCode::G1(Move {
            x: cut_pos,
            y: 0.0,
            z: drill_waiting,
            b: target_r - ((i * 2 + 1) as f64) * cfg.endmill.step,
            a: 360.0,
            feed_rate: cfg.endmill.feed_rate,
        }));
        gcodes.push(GCode::G1(Move {
            x: cut_pos,
            y: 0.0,
            z: drill_waiting,
            b: target_r - ((i * 2 + 2) as f64) * cfg.endmill.step,
            a: 0.0,
            feed_rate: cfg.endmill.feed_rate,
        }));
    }
    gcodes.push(GCode::G1(Move {
        x: cut_pos,
        y: 0.0,
        z: drill_waiting,
        b: drill_waiting,
        a: 0.0,
        feed_rate: cfg.feed_rate,
    }));
    gcodes
}

pub fn gen_gcode(mut proc: Proc, cfg: &CNCConfig) -> Result<String, Error> {
    proc.drills.sort_by(|x, y| if x.d > y.d { cmp::Ordering::Greater } else { cmp::Ordering::Less });
    let mut buf = String::new();
    let target_r = (proc.size.x().powi(2) + proc.size.y().powi(2)).sqrt();

    let mut gcodes = Vec::new();
    gcodes.push(GCode::Comment("init".to_owned()));
    gcodes.push(GCode::M02);

    for drill in proc.drills {
        gcodes.append(&mut gcodes_of_drill(&cfg, &drill, target_r));
    }
    gcodes.append(&mut gcodes_of_cut(cfg, cfg.gap_endmill_and_drill + cfg.endmill.r, target_r));
    gcodes.append(&mut gcodes_of_cut(cfg, proc.size.z() + cfg.gap_endmill_and_drill + cfg.endmill.r, target_r));
    gcodes.push(GCode::M03);
    output(&mut buf, cfg, &gcodes)?;
    Ok(buf)
}
