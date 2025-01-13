package A0.Java;

public class Rectangle extends Figure {
    // Rectangle attributes
    public int x1, y1;
    public int x2, y2;

    // Class constructor
    public Rectangle(int x1, int y1, int x2, int y2) {
        this.x1 = x1;
        this.y1 = y1;
        this.x2 = x2;
        this.y2 = y2;
    }

    /**
     * Scales a factor by a given factor
     * Rectangle - scales from the lower left corner
     */
    public void scale(int factor) {
        /* Rescaling by 0 shrinks the figure down to essentially a point */
        if (factor == 0) {
            x2 = x1;
            y2 = y1;
            return;
        }

        /* Rescaling by 1 leaves the figure unchanged */
        if (factor == 1) {
            return;
        }

        /* Rescaling by 2 makes the dimensions of the figure be doubled */
        x2 = x1 + (x2 - x1) * factor;
        y2 = y1 + (y2 - y1) * factor;
    }
    
    /**
     * Takes a pair of integers, and moves the Figure by the offse amounts given by the integers
     */
    public void xlate(int xOffset, int yOffset) {
        x1 = x1 + xOffset;
        x2 = x2 + xOffset;
        y1 = y1 + yOffset;
        y2 = y2 + yOffset;
    }
    
    /**
     * Prints a text representation of the figure
     * Rectangle - Rectangle@(x1, y1)*(x2, y2)
     */
    public void print() {
        /* Rectangle@(x1, y1)*(x2, y2) */
        System.out.printf("Rectangle@(%d,%d)*(%d,%d)%n", x1, y1, x2, y2);
    }
}
