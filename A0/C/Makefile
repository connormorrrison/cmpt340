CC = gcc
CFLAGS = -ansi -Wall -pedantic
OBJS = main.o figure.o
TARGET = main

.PHONY: all clean

all: $(TARGET)
	rm -f $(OBJS)

# Source code to object file
main.o: main.c figure.h
	$(CC) $(CFLAGS) -c main.c

# Source code to object file
figure.o: figure.c figure.h
	$(CC) $(CFLAGS) -c figure.c

# Object files to executable file
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJS)

clean:
	rm -f $(TARGET) $(OBJS)