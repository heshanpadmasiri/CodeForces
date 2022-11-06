SRC=src/main.rs
INPUT=$(wildcard *.in)
OUTPUT=$(wildcard *.out)
DIFF=$(patsubst %.in,%.diff, $(wildcard *.in))
TARGETS=build test clean
RUST=cargo

build: 
	$(RUST) build

test:$(DIFF)

clean:
	$(RUST) clean
	rm -f *.actual
	rm -f *.diff

%.diff: %.in %.out $(SRC)
	less $*.in | $(RUST) run > $*.actual		
	diff $*.actual $*.out > $*.diff 

.PHONY : $(TARGETS)
