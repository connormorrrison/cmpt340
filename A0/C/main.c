#include <stdio.h>
#include "figure.h"

int main(void) {
    /* TODO */
    /* Sample figures */
    Figure circle1 = {CIRCLE, .shape.circle = {0, 0, 1}};
    Figure rectangle1 = {RECTANGLE, .shape.rectangle = {-3, 2, -1, 3}};
    Figure line1 = {LINE, .shape.line = {1, -2}};

    /* Figures pre-transformation */
    printf("Figures pre-transformation:\n");

    /* Applying transformations */
    printf("Applying scale:\n");
    printf("Scale by 0:\n");
    scale(&circle1, 0);
    scale(&rectangle1, 0);
    scale(&line1, 0);

    printf("Scale by 1:\n");
    scale(&circle1, 1);
    scale(&rectangle1, 1);
    scale(&line1, 1);

    printf("Scale by 2:\n");
    scale(&circle1, 2);
    scale(&rectangle1, 2);
    scale(&line1, 2);

    /* Figures post-transformation */
    printf("Figures post-transformation:\n");

    return 0;
}
