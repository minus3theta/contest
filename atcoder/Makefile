CPPFLAGS=-O2 -Wall -std=gnu++1y

src=$(wildcard *.cpp)
targ=$(src:%.cpp=%)

all: $(targ)

%.run: %
	./$<
