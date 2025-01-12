#include <stdio.h>
#include "figure.h"

/**
 * Scales a factor by a given factor.
 * Circle - scales the radius.
 * Rectangle - scales from the lower left corner.
 * Lines -  scales the length.
 */
void scale(Figure *figure, int factor) {
    /* Rescaling by 0 shrinks the figure down to essentially a point */
    if (factor == 0) {
        switch (figure->type) {
            case CIRCLE:
                figure->circle.radius = 0;
                break;
            case RECTANGLE:
                /* Collapses points to the lower left (x, y) coordinates */
                figure->rectangle.x2 = figure->rectangle.x1;
                figure->rectangle.y2 = figure->rectangle.y1;
            case LINE:
                figure->line.length = 0;
                break;
        }
    }
    
    /* Rescaling by 1 leaves the figure unchanged */
    if (factor == 1) {
        return;
    }
    
    /* Rescaling by 2 makes the dimensions of the figure be doubled */
    if (factor > 1) {
        switch (figure->type) {
            case CIRCLE:
                figure->circle.radius = figure->circle.radius * factor;
                break;
            case RECTANGLE:
                figure->rectangle.x2 = figure->rectangle.x1 + (figure->rectangle.x2 - figure->rectangle.x1) * factor;
                figure->rectangle.y2 = figure->rectangle.y1 + (figure->rectangle.y2 - figure->rectangle.y1) * factor;
                break;
            case LINE:
                figure->line.length = figure->line.length * factor;
                break;
        }
    }
}

void xlate(Figure *figure, int xOffset, int yOffset);

void print(Figure *figure);
