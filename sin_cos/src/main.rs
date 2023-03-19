use std::f64::consts::PI;
use plotters::prelude::*;

fn main() {
    let x_deg = (0..=360).map(f64::from).collect::<Vec<_>>();
    let x_rad = x_deg.iter().map(|x| x * PI / 180.0).collect::<Vec<_>>();
    let y0 = x_rad.iter().map(|&x| x.sin()).collect::<Vec<_>>();
    let y1 = x_rad.iter().map(|&x| x.cos()).collect::<Vec<_>>();

    let root = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Sine and Cosine Functions", ("Arial", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0f64..360f64, -1f64..1f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(x_deg.iter().zip(&y0).map(|(&x, &y)| (x, y)), &BLUE))
        .unwrap()
        .label("Sine Function");

    chart
        .draw_series(LineSeries::new(x_deg.iter().zip(&y1).map(|(&x, &y)| (x, y)), &GREEN))
        .unwrap()
        .label("Cosine Function");

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
