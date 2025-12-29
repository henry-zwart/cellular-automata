use super::elementary::Grid;
use plotters::prelude::*;
use std::path::Path;

pub fn plot_evolution<const W: usize, P: AsRef<Path>>(
    generations: &[Grid<W>],
    filepath: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let cell_size = 10; // Size of each cell in pixels
    let width = W * cell_size; // Fixed width for each generation
    let height = generations.len() * cell_size;

    // Create a drawing area with the specified file path
    let root =
        BitMapBackend::new(filepath.as_ref(), (width as u32, height as u32)).into_drawing_area();

    root.fill(&WHITE)?;

    // Draw each cell
    for (row_idx, row) in generations.iter().enumerate() {
        for (col_idx, &state) in row.0.iter().enumerate() {
            let color = if state == 1 { &BLACK } else { &WHITE };
            root.draw(&Rectangle::new(
                [
                    ((col_idx * cell_size) as i32, (row_idx * cell_size) as i32),
                    (
                        ((col_idx + 1) * cell_size) as i32,
                        ((row_idx + 1) * cell_size) as i32,
                    ),
                ],
                color.filled(),
            ))?;
        }
    }

    root.present()?;
    println!("Plot saved to {:?}", filepath.as_ref());

    Ok(())
}
