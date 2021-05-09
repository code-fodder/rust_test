# This was "auto-generated" (output copied) from make_utils

.PHONY: all
all:
	mkdir -p dep/x64Linux_debug obj/x64Linux_debug lib
	g++ -Wall -Wextra -m64 -fPIC -I. -std=c++17 -MT obj/x64Linux_debug/./cutils.o -MMD -MP -MF dep/x64Linux_debug/./cutils.Td -c cutils.cpp -o obj/x64Linux_debug/./cutils.o
	gcc -m64 -shared obj/x64Linux_debug/./cutils.o -o lib/libadd_x64Linuxd.so   -lstdc++ -lpthread -lrt

.PHONY: clean
clean:
	rm -rf obj dep lib .build_prerequisites
