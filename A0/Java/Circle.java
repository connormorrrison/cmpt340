package A0.Java;

public class Circle extends Figure {
    // Circle attributes
    public int x, y;
    public int radius;
    
    // Class constructor
    public Circle(int x, int y, int radius) {
        this.x = x;
        this.y = y;
        this.radius = radius;
    }

    /**
     * Scales a factor by a given factor
     * Circle - scales the radius
     */
    public void scale(int factor) {
        /* Rescaling by 0 shrinks the figure down to essentially a point */
        if (factor == 0) {
            radius = 0;
            return;
        }

        /* Rescaling by 1 leaves the figure unchanged */
        if (factor == 1) {
            return;
        }

        /* Rescaling by 2 makes the dimensions of the figure be doubled */
        radius = radius * factor;
    }
    
    /**
     * Takes a pair of integers, and moves the Figure by the offse amounts given by the integers
     */
    public void xlate(int xOffset, int yOffset) {
        x = x + xOffset;
        y = y + yOffset;
    }
    
    public void print() {
        return;
    }
}
