EXE=./main
SRC=main.go
INPUTS=$(wildcard input*.txt)
OUTPUTS=$(wildcard output*.txt)
GEN_OUTPUTS=$(addprefix gen_, $(OUTPUTS))

test: $(GEN_OUTPUTS) $(OUTPUTS)
	@for f in $(OUTPUTS); do \
		echo $$f; \
		diff $$f gen_$$f; \
	done

gen_output%.txt: input%.txt $(EXE)
	less $< | $(EXE) | tee $@

$(EXE):$(SRC)
	go build $<

clean:
	rm -f $(EXE)
	rm -f $(GEN_OUTPUTS)

.PHONY: test clean
