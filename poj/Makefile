CPPFLAGS=-O2 -g -Wall -std=c++98

src=$(wildcard *.cpp)
targ=$(src:%.cpp=%)

all: $(targ)

%.run: %
	./$<
