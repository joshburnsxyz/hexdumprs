CARGO := $(shell which cargo)
BIN := hexdumprs
OUTDIR := dist
TARGETDIR := target
PREFIX := /usr/share

debug:
	mkdir -p $(OUTDIR)
	$(CARGO) build \
		--verbose
	mv $(TARGETDIR)/debug/$(BIN) $(OUTDIR)/$(BIN)	


release:
	mkdir -p $(OUTDIR)
	$(CARGO) build \
		--verbose \
		--release
	mv $(TARGETDIR)/release/$(BIN) $(OUTDIR)/$(BIN)	

clean:
	@rm -rf $(OUTDIR)
	@rm -rf $(TARGETDIR)

.PHONY: clean release debug
