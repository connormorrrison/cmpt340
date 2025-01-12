package A0.Java;

public class Line extends Figure {
    // Line attributes
    public int x, y;
    public int angle;
    public int length;

    // Class constructor
    public Line(int x, int y, int angle, int length) {
        this.x = x;
        this.y = y;
        this.angle = angle;
        this.length = length;
    }

    /**
     * Scales a factor by a given factor
     * Lines - scales the length
     */
    public void scale(int factor) {
        /* Rescaling by 0 shrinks the figure down to essentially a point */
        if (factor == 0) {
            length = 0;
            return;
        }

        /* Rescaling by 1 leaves the figure unchanged */
        if (factor == 1) {
            return;
        }

        /* Rescaling by 2 makes the dimensions of the figure be doubled */
        length = length * factor;
    }
    
    /**
     * Takes a pair of integers, and moves the Figure by the offse amounts given by the integers
     */
    public void xlate(int xOffset, int yOffset) {
        x = x + xOffset;
        y = y + yOffset;
    }
    
    /**
     * Prints a text representation of the figure
     * Line - Line@(x, y) with angle a° and length l
     */
    public void print() {
        /* Line@(x, y) with angle a° and length l */
        System.out.printf("Line@(%d, %d) with angle %d° and length %d%n", x, y, angle, length);
    }
}
