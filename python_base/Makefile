SRC=$(wildcard *.py)
INPUT=$(wildcard *.in)
OUTPUT=$(wildcard *.out)
DIFF=$(patsubst %.in,%.diff, $(wildcard *.in))
TARGETS=test clean

test:$(DIFF)

clean:
	rm -f *.actual
	rm -f *.diff

%.diff: %.in %.out $(SRC)
	less $*.in | python3 $(SRC) > $*.actual
	diff $*.actual $*.out > $*.diff

.PHONY : $(TARGETS)
