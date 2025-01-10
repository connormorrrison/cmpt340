CC = gcc
CFLAGS = -ansi -Wall -pedantic
OBJS = main.o figure.o
TARGET = main

.PHONY: all clean

all: $(TARGET)

# source code to object file
main.o: main.c
	$(CC) $(CFLAGS) -c main.c

# source code to object file
figure.o: figure.c
	$(CC) $(CFLAGS) -c figure.c

# object files to executable file
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJS)

clean:
	rm -f $(TARGET) $(OBJS)