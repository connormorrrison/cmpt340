#include <stdio.h>
#include "figure.h"

int main(void) {
    /* Sample figures */
    Figure figuresArray[] = {
        {CIRCLE, .shape.circle = {0, 0, 1}},
        {RECTANGLE, .shape.rectangle = {-3, 2, -1, 3}},
        {LINE, .shape.line = {1, -2, 45, 1}}
    };

    /* Reset figures to initial values */
    Figure figuresArrayReset[] = {
        {CIRCLE, .shape.circle = {0, 0, 1}},
        {RECTANGLE, .shape.rectangle = {-3, 2, -1, 3}},
        {LINE, .shape.line = {1, -2, 45, 1}}
    };

    int numFigures = sizeof(figuresArray) / sizeof(figuresArray[0]);
    int i;

    /* Print initial figures */
    printf("Initial figures:\n");
    for (i = 0; i < numFigures; i++) {
        print(&figuresArray[i]);
    }

    /* Scale by 0 */
    printf("\nScaling figures by 0:\n");
    for (i = 0; i < numFigures; i++) {
        scale(&figuresArray[i], 0);
        print(&figuresArray[i]);
    }

    /* Reset figures to initial values */
    for (i = 0; i < numFigures; i++) {
        figuresArray[i] = figuresArrayReset[i];
    }

    /* Scale by 1 */
    printf("\nScaling figures by 1:\n");
    for (i = 0; i < numFigures; i++) {
        scale(&figuresArray[i], 1);
        print(&figuresArray[i]);
    }

    /* Reset figures to initial values */
    for (i = 0; i < numFigures; i++) {
        figuresArray[i] = figuresArrayReset[i];
    }

    /* Scale by 2 */
    printf("\nScaling figures by 2:\n");
    for (i = 0; i < numFigures; i++) {
        scale(&figuresArray[i], 2);
        print(&figuresArray[i]);
    }

    return 0;
}
