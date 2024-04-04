use std::{fmt::Debug, num, ops::Range};

use anyhow::Result;
use num_traits::Num;
use plotters::{coord::ranged1d::ValueFormatter, prelude::*};

pub fn plot_result(x_range: Range<f32>, y_range: Range<f32>, data: Vec<(f32, f32)>) -> Result<()> {
    let root = BitMapBackend::new("subterfuge.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("defenders = 10", ("sans-serif", 50))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range, y_range)?;

    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(data, &RED))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED))
        .label("asdf");
    Ok(())
}

pub fn plot() -> Result<()> {
    let root = BitMapBackend::new("wtf.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plot() {
        plot().unwrap();
    }
}
