// Define global constants
const ORIGIN_X: i32 = 0;
const ORIGIN_Y: i32 = 0;

// Define enum
enum Figure {
    Circle(i32, i32, i32),  // x, y, radius
    Rectangle(i32, i32, i32, i32),  // x1, y1, x2, y2
    Line(i32, i32, i32, i32)  // x, y, angle, length
}

impl Figure {
    // Scales a figure by a given factor
    fn scale(&mut self, factor: i32) {
        // Rescaling by 0 shrinks the figure down to essentially a point
        if factor == 0 {
            match self {
                Figure::Circle(_, _, radius) => {
                    *radius = 0;  // Shrinks to a point
                }
                Figure::Rectangle(x1, y1, x2, y2) => {
                    *x2 = *x1;  // Collapses points to the lower left (x, y) coordinates
                    *y2 = *y1;
                }
                Figure::Line(_, _, _, length) => {
                    *length = 0;  // Shrinks to a point
                }
            }
            return;
        }
        // Rescaling by 1 leaves the figure unchanged
        if factor == 1 {
            return;
        }

        // Rescaling by 2 makes the dimensions of the figure be doubled
        match self {
            Figure::Circle(_, _, radius) => {
                *radius *= factor;
            }
            Figure::Rectangle(x1, y1, x2, y2) => {
                *x2 = *x1 + (*x2 - *x1) * factor;
                *y2 = *y1 + (*y2 - *y1) * factor;
            }
            Figure::Line(_, _, _, length) => {
                *length *= factor;
            }
        }
    }

    // Takes a pair of integers, and moves the Figure by the offset amounts given by the integers
    fn xlate(&mut self, x_offset: i32, y_offset: i32) {
        match self {
            Figure::Circle(x, y, _) => {
                *x += x_offset;
                *y += y_offset;
            }
            Figure::Rectangle(x1, y1, x2, y2) => {
                *x1 += x_offset;
                *y1 += y_offset;
                *x2 += x_offset;
                *y2 += y_offset;
            }
            Figure::Line(x, y, _, _) => {
                *x += x_offset;
                *y += y_offset;
            }
        }
    }

    fn print(&self) {
        match self {
            Figure::Circle(x, y, radius) => {
                println!("Circle@({}, {}) with radius {}", x, y, radius);
            }
            Figure::Rectangle(x1, y1, x2, y2) => {
                println!("Rectangle@({}, {})*({}, {})", x1, y1, x2, y2);
            }
            Figure::Line(x, y, angle, length) => {
                println!("Line@({}, {}) with angle {}° and length {}", x, y, angle, length);
            }
        }
    }
}

fn main() {
    // Declare an array of figures
    let mut figures: [Figure; 3] = [
        Figure::Circle(ORIGIN_X, ORIGIN_Y, 1),
        Figure::Rectangle(-3, 2, -1, 3),
        Figure::Line(1, -2, 45, 1),
    ];

    println!("Figures before transformations:");
    for figure in figures.iter() {
        figure.print();
    }

    // Apply transformations
    println!("\nFigures after transformations:");
    for figure in figures.iter_mut() {
        figure.xlate(2, 3);
        figure.scale(2);
        figure.print();
    }
}
