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
    // Circle: scales the radius
    // Rectangle: scales from the lower left corner
    // Lines: scales the length
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
                *x1 = *x1 + (*x2 - *x1) * factor;
                *y2 = *y1 + (*y2 - *y1) * factor;
            }
            Figure::Line(_, _, _, length) => {
                *length *= factor;
            }
        }
    }

    // Takes a pair of integers, and moves the Figure by the offset amounts given by the integers
    fn xlate(&mut self, xOffset: i32, yOffset: i32) {
        match self {
            Figure::Circle(x, y, _) => {
                *x += *xOffset;
                *y += *yOffset;
            }
            Figure::Rectangle(x1, y1, x2, y2) => {
                *x1 += xOffset;
                *y1 += yOffset;
                *x2 += xOffset;
                *y2 += yOffset;
            }
            Figure::Line(x, y, _, _) => {
                *x += xOffset;
                *y += yOffset;
            }
        }
    }

    fn print(&mut self) {
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
    // Sample
    println!("Hello, world!");
}
