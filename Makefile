CARGO := $(shell which cargo)
BIN := hexdumprs
OUTDIR := dist
TARGETDIR := target/debug
PREFIX := /usr/share

hexdumprs:
	mkdir -p $(OUTDIR)
	$(CARGO) build \
		--verbose
	mv $(TARGETDIR)/$(BIN) $(OUTDIR)/$(BIN)	

clean:
	@rm -rf $(OUTDIR)
	@rm -rf target

.PHONY: clean
