SRC=$(wildcard *.cpp)
EXE=$(patsubst %.cpp,%.exe, $(SRC))
INPUT=$(wildcard *.in)
OUTPUT=$(wildcard *.out)
DIFF=$(patsubst %.in,%.diff, $(wildcard *.in))
TARGETS=build test clean

build: $(EXE) 

test:$(DIFF)

clean:
	rm -f *.exe
	rm -f *.actual
	rm -f *.diff

%.exe: %.cpp
	g++ $< -o $@

%.diff: %.in %.out $(EXE)
	less $*.in | ./$(EXE) > $*.actual		
	diff $*.actual $*.out > $*.diff 

.PHONY : $(TARGETS)
