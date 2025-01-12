#include <stdio.h>
#include "figure.h"

int main(void) {
    /* TODO */
    /* Sample figures */
    Figure circle1 = {CIRCLE, .circle = {0, 0, 1}};
    Figure rectangle1 = {RECTANGLE, .rectangle = {-3, 2, -1, 3}};
    Figure line1 = {LINE, .line = {1, -2}};

    /* Figures pre-transformation */
    printf("Figures pre-transformation:\n");

    return 0;
}
