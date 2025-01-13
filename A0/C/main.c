#include <stdio.h>
#include "figure.h"

int main(void) {
    int numFigures = 3;
    int i;

    /* Sample figures */
    Figure figuresArray[3];

    /* Initialize Circle */
    figuresArray[0].type = CIRCLE;
    figuresArray[0].shape.circle.x = ORIGIN_X;
    figuresArray[0].shape.circle.y = ORIGIN_Y;
    figuresArray[0].shape.circle.radius = 1;

    /* Initialize Rectangle */
    figuresArray[1].type = RECTANGLE;
    figuresArray[1].shape.rectangle.x1 = -3;
    figuresArray[1].shape.rectangle.y1 = 2;
    figuresArray[1].shape.rectangle.x2 = -1;
    figuresArray[1].shape.rectangle.y2 = 3;

    /* Initialize Line */
    figuresArray[2].type = LINE;
    figuresArray[2].shape.line.x = 1;
    figuresArray[2].shape.line.y = -2;
    figuresArray[2].shape.line.angle = 45;
    figuresArray[2].shape.line.length = 1;

    /* Initial figures */
    printf("Figures before transformations:\n");
    for (i = 0; i < numFigures; i++) {
        print(&figuresArray[i]);
    }

    /* Figures after transformation xlate by 2, 3 and scale by 2 */
    printf("\nFigures after transformations:\n");
    for (i = 0; i < numFigures; i++) {
        xlate(&figuresArray[i], 2, 3);
        scale(&figuresArray[i], 2);
        print(&figuresArray[i]);
    }

    return 0;
}
