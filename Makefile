CFLAGS=-std=c11 -g -fno-common

test:
	./test.sh

clean:
	rm -f *.o *~ tmp*

.PHONY: test clean
