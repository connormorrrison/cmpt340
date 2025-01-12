package A0.Java;

public abstract class Figure {
    public static final int ORIGIN_X = 0;
    public static final int ORIGIN_Y = 0;

    public abstract void scale(int factor);
    public abstract void xlate(int xOffset, int yOffset);
    public abstract void print();
}
