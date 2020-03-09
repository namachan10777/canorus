use super::analysis::Proc;
use std::cmp;
use std::fmt::{Write, Error};

const DRILL_PULLING : f64 = 30.0;
const ENDMILL_R : f64 = 3.0;
const GAP_ENDMILL_AND_DRILL : f64 = 40.0;
const FEED_RATE : f64 = 100.0;
const EPS : f64 = 1e-10;
const X_OFFSET: f64 = 0.0;
const A_OFFSET: f64 = 0.0;
const Z_OFFSET: f64 = 0.0;
const Y_OFFSET: f64 = 0.0;
const ENDMILL_STEP : f64 = 0.05;
const DRILL_OFFSET : f64 = 10.0;
const D_PULLING : f64 = 40.0;

fn g01(buf: &mut String, x: f64, y: f64, z: f64, a: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("G01 X{:.3} Y{:.3} Z{:.3} A{:.3} F{:.3}\n",
        x + X_OFFSET + EPS,
        y + Y_OFFSET + EPS,
        z + Z_OFFSET + EPS,
        a + A_OFFSET + EPS,
        FEED_RATE + EPS))
}

fn g00(buf: &mut String, x: f64, y: f64, z: f64, a: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("G00 X{:.3} Y{:.3} Z{:.3} A{:.3}\n",
        x + X_OFFSET + EPS,
        y + Y_OFFSET + EPS,
        z + Z_OFFSET + EPS,
        a + A_OFFSET + EPS))
}

fn gen_cut(buf: &mut String, cut_pos: f64, target_r: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("; cut\n"))?;
    g01(buf, 0.0, cut_pos, target_r + DRILL_OFFSET, 0.0)?;
    g01(buf, 0.0, cut_pos, target_r, 0.0)?;
    let iter_times = (target_r / ENDMILL_STEP / 2.0).ceil() as i32;
    for i in 0..iter_times {
        g01(buf, 0.0, cut_pos, target_r - ((i * 2)      as f64) * ENDMILL_STEP, 3.15)?;
        g01(buf, 0.0, cut_pos, target_r - ((i * 2 + 1)  as f64) * ENDMILL_STEP, 0.00)?;
    }
    g01(buf, 0.0, cut_pos, DRILL_PULLING, 0.00)?;
    Ok(())
}

fn gen_drill(buf: &mut String, slide: f64, d: f64, theta: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("; drill\n"))?;
    g01(buf, slide, d, DRILL_PULLING, theta)?;
    g01(buf, slide, d, 0.0,           theta)?;
    g01(buf, slide, d, DRILL_PULLING, theta)?;
    Ok(())
}

fn gen_reset(buf: &mut String, d: f64) -> Result<(), Error> {
    buf.write_fmt(format_args!("; reset\n"))?;
    g00(buf, 0.0, d - D_PULLING, DRILL_PULLING, 0.0)?;
    buf.write_fmt(format_args!("M02\n"))?;
    Ok(())
}

fn gen_init(buf: &mut String) -> Result<(), Error> {
    buf.write_fmt(format_args!("; init\n"))?;
    buf.write_fmt(format_args!("M03\n"))?;
    Ok(())
}

pub fn gen_gcode(mut proc: Proc) -> Result<String, Error> {
    proc.drills.sort_by(|x, y| if x.d > y.d { cmp::Ordering::Greater } else { cmp::Ordering::Less });
    let mut buf = String::new();
    let target_r = (proc.size.x().powi(2) + proc.size.y().powi(2)).sqrt();
    gen_init(&mut buf)?;
    for drill in proc.drills {
        gen_drill(&mut buf, drill.slide, drill.d, drill.theta)?;
    }
    //gen_cut(&mut buf, GAP_ENDMILL_AND_DRILL + ENDMILL_R, target_r)?;
    //gen_cut(&mut buf, proc.size.z() + GAP_ENDMILL_AND_DRILL + ENDMILL_R, target_r)?;
    gen_reset(&mut buf, proc.size.z())?;
    Ok(buf)
}
