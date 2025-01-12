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

/* Figure types */
typedef enum {
    CIRCLE,
    RECTANGLE,
    LINE
} FigureType;

/* A generic figure */
typedef struct {
    FigureType type;
    union {
        Circle circle;
        Rectangle rectangle;
        Line line;
    } shape;
} Figure;

/* Function declarations */
void scale(Figure *figure, int factor);
void xlate(Figure *figure, int xOffset, int yOffset);
void print(Figure *figure);

#endif
