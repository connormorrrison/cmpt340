#ifndef FIGURE_H
#define FIGURE_H

/* Immutable origins */
#define ORIGIN_X 0
#define ORIGIN_Y 0

/* Circle */
typedef struct {
    int x, y;  /* Center of circle */
    int radius;
} Circle;

/* Rectangle */
typedef struct {
    int x1, y1;  /* Lower-left vertex */
    int x2, y2;  /* Upper-right vertex */
} Rectangle;

/* Line */
typedef struct {
    int x, y;  /* Lower-left vertex */
    int angle;
    int length;
} Line;

/* Function declarations */
void scale(, int factor);
void xlate(, int xOffset, int yOffset);
void print();

#endif
