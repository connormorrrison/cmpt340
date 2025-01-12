#include <stdio.h>
#include "figure.h"

int main(void) {
    /* Sample figures */
    Figure figuresArray[] = {
        {CIRCLE, .shape.circle = {ORIGIN_X, ORIGIN_Y, 1}},
        {RECTANGLE, .shape.rectangle = {-3, 2, -1, 3}},
        {LINE, .shape.line = {1, -2, 45, 1}}
    };

    int numFigures = sizeof(figuresArray) / sizeof(figuresArray[0]);
    int i;

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
