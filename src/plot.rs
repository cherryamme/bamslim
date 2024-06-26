
use std::collections::HashMap;
use plotters::prelude::*;
use std::path::Path;


pub fn plotter_coverages(outdir: &String,chrom: String,raw_coverages1: &HashMap<u32, u32>, raw_coverages2: &HashMap<u32, u32>) -> Result<(), Box<dyn std::error::Error>> {
	let outfig = Path::new(&outdir).join(format!("slim_{}.png",chrom));
	let root = BitMapBackend::new(&outfig, (640, 480)).into_drawing_area();
	let min_key = raw_coverages1.keys().min().unwrap().to_owned() as i32;
	let max_key = raw_coverages1.keys().max().unwrap().to_owned() as i32;
    let max_value = raw_coverages1.values().cloned().max().unwrap().to_owned() as i32;
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Bamslim Depth: {}",chrom), ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_key..max_key, 0..(1.1*max_value as f32) as i32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(raw_coverages1.iter().map(|(x, y)| Circle::new((*x as i32, *y as i32), 2, ShapeStyle::from(&BLUE).filled())))?.label("raw bam").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart.draw_series(raw_coverages2.iter().map(|(x, y)| Circle::new((*x as i32, *y as i32), 1, ShapeStyle::from(&RED.mix(0.5)).filled())))?.label("slim bam").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.5))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
