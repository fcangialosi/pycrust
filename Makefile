all:
	gcc -c -fpic $(shell /usr/bin/python2.7-config --cflags) middle.c $(shell /usr/bin/python2.7-config --ldflags)
	gcc -shared -Wl,-soname,libmultiply.so -o libmultiply.so middle.o $(shell /usr/bin/python2.7-config --ldflags)

clean:
	rm -f *.o *.pyc libmultiply.so
