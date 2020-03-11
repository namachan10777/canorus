use super::analysis::Proc;
use std::cmp;
use std::fmt::{Write, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Offsets {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CNCConfig {
    drill_pulling: f64,
    gap_endmill_and_drill: f64,
    feed_rate: f64,
    offsets: Offsets,
    endmill_step: f64,
    drill_offset: f64,
    y_pulling : f64,
}

fn g1(buf: &mut String, cfg: &CNCConfig, x: f64, y: f64, z: f64, a: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("G1 X{:.3} Y{:.3} Z{:.3} A{:.3} F{:.3}\n",
        x + cfg.offsets.x,
        y + cfg.offsets.y,
        z + cfg.offsets.z,
        a + cfg.offsets.a,
        cfg.feed_rate))
}

fn g0(buf: &mut String, cfg: &CNCConfig, x: f64, y: f64, z: f64, a: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("G0 X{:.3} Y{:.3} Z{:.3} A{:.3}\n",
        x + cfg.offsets.x,
        y + cfg.offsets.y,
        z + cfg.offsets.z,
        a + cfg.offsets.a))
}

fn gen_cut(buf: &mut String, cfg: &CNCConfig, cut_pos: f64, target_r: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("; cut\n"))?;
    g1(buf, cfg, 0.0, cut_pos, target_r + cfg.drill_offset, 0.0)?;
    g1(buf, cfg, 0.0, cut_pos, target_r, 0.0)?;
    let iter_times = (target_r / cfg.endmill_step / 2.0).ceil() as i32;
    for i in 0..iter_times {
        g1(buf, cfg, 0.0, cut_pos, target_r - ((i * 2)      as f64) * cfg.endmill_step, 3.15)?;
        g1(buf, cfg, 0.0, cut_pos, target_r - ((i * 2 + 1)  as f64) * cfg.endmill_step, 0.00)?;
    }
    g1(buf, cfg, 0.0, cut_pos, cfg.drill_pulling, 0.00)?;
    Ok(())
}

fn gen_drill(buf: &mut String, cfg: &CNCConfig, slide: f64, d: f64, theta: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("; drill\n"))?;
    g1(buf, cfg, slide, d, cfg.drill_pulling, theta)?;
    g1(buf, cfg, slide, d, 0.0,           theta)?;
    g1(buf, cfg, slide, d, cfg.drill_pulling, theta)?;
    Ok(())
}

fn gen_reset(buf: &mut String, cfg: &CNCConfig, d: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("; reset\n"))?;
    g0(buf, cfg, 0.0, d - cfg.y_pulling, cfg.drill_pulling, 0.0)?;
    buf.write_fmt(format_args!("M02\n"))?;
    Ok(())
}

fn gen_init(buf: &mut String, cfg: &CNCConfig) -> Result<(), Error> {
    buf.write_fmt(format_args!("; init\n"))?;
    buf.write_fmt(format_args!("M03\n"))?;
    Ok(())
}

pub fn gen_gcode(mut proc: Proc, cfg: &CNCConfig) -> Result<String, Error> {
    proc.drills.sort_by(|x, y| if x.d > y.d { cmp::Ordering::Greater } else { cmp::Ordering::Less });
    let mut buf = String::new();
    let target_r = (proc.size.x().powi(2) + proc.size.y().powi(2)).sqrt();
    gen_init(&mut buf, cfg)?;
    for drill in proc.drills {
        gen_drill(&mut buf, cfg, drill.slide, drill.d, drill.theta)?;
    }
    //gen_cut(&mut buf, GAP_ENDMILL_AND_DRILL + ENDMILL_R, target_r)?;
    //gen_cut(&mut buf, proc.size.z() + GAP_ENDMILL_AND_DRILL + ENDMILL_R, target_r)?;
    gen_reset(&mut buf, cfg, proc.size.z())?;
    Ok(buf)
}
