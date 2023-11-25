use std::{
    error::Error,
    fs,
    ops::Range,
    thread::{self},
};

use noise::{NoiseFn, Perlin};
use plotters::prelude::*;
use rand::*;

const OUT_DIR: &str = "out";

const DEFAULT_SURFACE_LEVEL: f64 = 2.0;
const DEFAULT_PERLIN_SEED: u32 = 1;
const DEFAULT_AMPLITUDE_REDUCE: f64 = 4.0;

const DEFAULT_GIF_W: u32 = 500;
const DEFAULT_GIF_H: u32 = 500;
const DEFAULT_GIF_FRAME_DELAY: u32 = 50;
const DEFAULT_GIF_MAX_PITCH: u32 = 157;
const DEFAULT_GIF_PITCH_SPEED_REDUCE: f64 = 50.0;
const DEFAULT_GIF_X_SPEC: Range<f64> = -10.0..10.0;
const DEFAULT_GIF_Y_SPEC: Range<f64> = 0.0..6.0;
const DEFAULT_GIF_Z_SPEC: Range<f64> = -10.0..10.0;

fn flat(_x: f64, _y: f64, surface_level: f64) -> f64 {
    surface_level
}

fn true_random(_x: f64, _y: f64, surface_level: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let randomness: f64 = rng.gen();
    surface_level + randomness
}

fn sine_curve(x: f64, y: f64, surface_level: f64, wave_l_incr: f64, ampl_red: f64) -> f64 {
    surface_level + ((x / wave_l_incr).sin() + (y / wave_l_incr).sin()) / ampl_red
}

fn perlin(
    x: f64,
    y: f64,
    surface_level: f64,
    wave_l_incr: f64,
    ampl_red: f64,
    perlin_seed: u32,
) -> f64 {
    let perlin = Perlin::new(perlin_seed);
    surface_level + perlin.get([x / wave_l_incr, y / wave_l_incr]) / ampl_red
}

fn plot_to_gif(
    f: fn(f64, f64) -> f64,
    out_file_name: &str,
    w: u32,
    h: u32,
    frame_delay: u32,
    max_pitch: u32,
    pitch_speed_reduce: f64,
    x_spec: Range<f64>,
    y_spec: Range<f64>,
    z_spec: Range<f64>,
) -> Result<(), Box<dyn Error>> {
    const DEFAULT_SCALE: f64 = 0.7;

    let root = BitMapBackend::gif(out_file_name, (w, h), frame_delay)?.into_drawing_area();

    let pitch_base: f64 = max_pitch as f64 / 100.0;
    for pitch in 0..max_pitch {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root).build_cartesian_3d(
            x_spec.clone(),
            y_spec.clone(),
            z_spec.clone(),
        )?;
        chart.with_projection(|mut p| {
            p.pitch = pitch_base - (pitch_base - pitch as f64 / pitch_speed_reduce).abs();
            p.scale = DEFAULT_SCALE;
            p.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        const STEP_REDUCE: f64 = 5.0;
        chart.draw_series(SurfaceSeries::xoz(
            ((x_spec.start * STEP_REDUCE) as i64..=(x_spec.end * STEP_REDUCE) as i64)
                .map(|x| x as f64 / STEP_REDUCE),
            ((z_spec.start * STEP_REDUCE) as i64..=(z_spec.end * STEP_REDUCE) as i64)
                .map(|x| x as f64 / STEP_REDUCE),
            f,
        ))?;
        root.present()?;
    }
    println!("Result has been saved to {out_file_name}");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(OUT_DIR)?;

    let threads = [
        thread::spawn(|| {
            plot_to_gif(
                |x, y| flat(x, y, DEFAULT_SURFACE_LEVEL),
                "out/flat.gif",
                DEFAULT_GIF_W,
                DEFAULT_GIF_H,
                DEFAULT_GIF_FRAME_DELAY,
                DEFAULT_GIF_MAX_PITCH,
                DEFAULT_GIF_PITCH_SPEED_REDUCE,
                DEFAULT_GIF_X_SPEC,
                DEFAULT_GIF_Y_SPEC,
                DEFAULT_GIF_Z_SPEC,
            )
            .unwrap();
        }),
        thread::spawn(|| {
            plot_to_gif(
                |x, y| true_random(x, y, DEFAULT_SURFACE_LEVEL),
                "out/flat_random.gif",
                DEFAULT_GIF_W,
                DEFAULT_GIF_H,
                DEFAULT_GIF_FRAME_DELAY,
                DEFAULT_GIF_MAX_PITCH,
                DEFAULT_GIF_PITCH_SPEED_REDUCE,
                DEFAULT_GIF_X_SPEC,
                DEFAULT_GIF_Y_SPEC,
                DEFAULT_GIF_Z_SPEC,
            )
            .unwrap();
        }),
        thread::spawn(|| {
            plot_to_gif(
                |x, y| sine_curve(x, y, DEFAULT_SURFACE_LEVEL, 1.0, DEFAULT_AMPLITUDE_REDUCE),
                "out/sine_curve.gif",
                DEFAULT_GIF_W,
                DEFAULT_GIF_H,
                DEFAULT_GIF_FRAME_DELAY,
                DEFAULT_GIF_MAX_PITCH,
                DEFAULT_GIF_PITCH_SPEED_REDUCE,
                DEFAULT_GIF_X_SPEC,
                DEFAULT_GIF_Y_SPEC,
                DEFAULT_GIF_Z_SPEC,
            )
            .unwrap();
        }),
        thread::spawn(|| {
            plot_to_gif(
                |x, y| sine_curve(x, y, DEFAULT_SURFACE_LEVEL, 4.0, DEFAULT_AMPLITUDE_REDUCE),
                "out/sine_curve_long.gif",
                DEFAULT_GIF_W,
                DEFAULT_GIF_H,
                DEFAULT_GIF_FRAME_DELAY,
                DEFAULT_GIF_MAX_PITCH,
                DEFAULT_GIF_PITCH_SPEED_REDUCE,
                DEFAULT_GIF_X_SPEC,
                DEFAULT_GIF_Y_SPEC,
                DEFAULT_GIF_Z_SPEC,
            )
            .unwrap();
        }),
        thread::spawn(|| {
            plot_to_gif(
                |x, y| {
                    perlin(
                        x,
                        y,
                        DEFAULT_SURFACE_LEVEL,
                        1.0,
                        DEFAULT_AMPLITUDE_REDUCE,
                        DEFAULT_PERLIN_SEED,
                    )
                },
                "out/perlin.gif",
                DEFAULT_GIF_W,
                DEFAULT_GIF_H,
                DEFAULT_GIF_FRAME_DELAY,
                DEFAULT_GIF_MAX_PITCH,
                DEFAULT_GIF_PITCH_SPEED_REDUCE,
                DEFAULT_GIF_X_SPEC,
                DEFAULT_GIF_Y_SPEC,
                DEFAULT_GIF_Z_SPEC,
            )
            .unwrap();
        }),
        thread::spawn(|| {
            plot_to_gif(
                |x, y| {
                    perlin(
                        x,
                        y,
                        DEFAULT_SURFACE_LEVEL,
                        4.0,
                        DEFAULT_AMPLITUDE_REDUCE,
                        DEFAULT_PERLIN_SEED,
                    )
                },
                "out/perlin_long.gif",
                DEFAULT_GIF_W,
                DEFAULT_GIF_H,
                DEFAULT_GIF_FRAME_DELAY,
                DEFAULT_GIF_MAX_PITCH,
                DEFAULT_GIF_PITCH_SPEED_REDUCE,
                DEFAULT_GIF_X_SPEC,
                DEFAULT_GIF_Y_SPEC,
                DEFAULT_GIF_Z_SPEC,
            )
            .unwrap();
        }),
    ];

    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}
