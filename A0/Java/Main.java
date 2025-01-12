package A0.Java;

public class Main {
    public static void main(String[] args) {
        
        // Sample figures
        Figure[] figuresArray = {
            new Circle(Figure.ORIGIN_X, Figure.ORIGIN_Y, 1),
            new Rectangle(-3, 2, -1, 3),
            new Line(1, -2, 45, 1)
        };

        // Initial figures
        System.out.println("Figures before transformations:");
        for (Figure figure : figuresArray) {
            figure.print();
        }

        // Figures after transformation xlate by 2, 3 and scale by 2
        System.out.println("\nFigures after transformations:");
        for (Figure figure : figuresArray) {
            figure.xlate(2, 3);
            figure.scale(2);
            figure.print();
        }
    }
}
