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

    public void scale(int factor) {
        return;
    }
    
    public void xlate(int xOffset, int yOffset) {
        return;
    }
    
    public void print() {
        return;
    }
}
