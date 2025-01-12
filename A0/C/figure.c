#include <stdio.h>
#include "figure.h"

/**
 * Scales a factor by a given factor
 * Circle - scales the radius
 * Rectangle - scales from the lower left corner
 * Lines - scales the length
 */
void scale(Figure *figure, int factor) {
    /* Rescaling by 0 shrinks the figure down to essentially a point */
    if (factor == 0) {
        switch (figure->type) {
            case CIRCLE:
                figure->shape.circle.radius = 0;
                break;
            case RECTANGLE:
                /* Collapses points to the lower left (x, y) coordinates */
                figure->shape.rectangle.x2 = figure->shape.rectangle.x1;
                figure->shape.rectangle.y2 = figure->shape.rectangle.y1;
                break;
            case LINE:
                figure->shape.line.length = 0;
                break;
        }
        return;
    }
    
    /* Rescaling by 1 leaves the figure unchanged */
    if (factor == 1) {
        return;
    }
    
    /* Rescaling by 2 makes the dimensions of the figure be doubled */
    switch (figure->type) {
        case CIRCLE:
            figure->shape.circle.radius = figure->shape.circle.radius * factor;
            break;
        case RECTANGLE:
            figure->shape.rectangle.x2 = figure->shape.rectangle.x1 + (figure->shape.rectangle.x2 - figure->shape.rectangle.x1) * factor;
            figure->shape.rectangle.y2 = figure->shape.rectangle.y1 + (figure->shape.rectangle.y2 - figure->shape.rectangle.y1) * factor;
            break;
        case LINE:
            figure->shape.line.length = figure->shape.line.length * factor;
            break;
    }
}

/**
 * Takes a pair of integers, and moves the Figure by the offse amounts given by the integers
 */
void xlate(Figure *figure, int xOffset, int yOffset) {
    switch (figure->type) {
        case CIRCLE:
            figure->shape.circle.x = figure->shape.circle.x + xOffset;
            figure->shape.circle.y = figure->shape.circle.y + yOffset;
            break;
        case RECTANGLE:
            figure->shape.rectangle.x1 = figure->shape.rectangle.x1 + xOffset;
            figure->shape.rectangle.x2 = figure->shape.rectangle.x2 + xOffset;
            figure->shape.rectangle.y1 = figure->shape.rectangle.y1 + yOffset;
            figure->shape.rectangle.y2 = figure->shape.rectangle.y2 + yOffset;
            break;
        case LINE:
            figure->shape.line.x = figure->shape.line.x + xOffset;
            figure->shape.line.y = figure->shape.line.y + yOffset;
            break;
    }
}

/**
 * Prints a text representation of the figure
 * Circle - Circle@(x, y) with radius r
 * Rectangle - Rectangle@(x1, y1)*(x2, y2)
 * Line - Line@(x, y) with angle a° and length l
 */
void print(Figure *figure) {
    switch (figure->type) {
        case CIRCLE:
            /* Circle@(x, y) with radius r */
            printf("Circle@(%d, %d) with radius %d\n",
            figure->shape.circle.x,
            figure->shape.circle.y,
            figure->shape.circle.radius);
            break;
        case RECTANGLE:
            /* Rectangle@(x1, y1)*(x2, y2) */
            printf("Rectangle@(%d, %d)*(%d, %d)\n",
            figure->shape.rectangle.x1,
            figure->shape.rectangle.y1,
            figure->shape.rectangle.x2,
            figure->shape.rectangle.y2);
            break;
        case LINE:
            /* Line@(x, y) with angle a° and length l */
            printf("Line@(%d, %d) with angle %d° and length %d\n",
            figure->shape.line.x,
            figure->shape.line.y,
            figure->shape.line.angle,
            figure->shape.line.length);
            break;
    }
}
