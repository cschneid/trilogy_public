SRC ?= src
BUILD ?= build
RUSTC ?= /usr/local/bin/rustc
ARCH ?= $(shell $(RUSTC) -v | grep host | cut -f 2 -d ' ')
LIBDIR ?= $(HOME)/.rust/lib
PROJNAME ?= $(subst rust-,,$(shell basename $(CURDIR)))
LIBSRC ?= $(SRC)/$(PROJNAME)/lib.rs
LIB ?= $(BUILD)/lib$(PROJNAME)-*.rlib
EXAMPLESRCS ?= $(wildcard $(SRC)/examples/*.rs)

.PHONY: all clean cleandeps deps lib test install examples

all: test

$(BUILD):
	@mkdir -p $(BUILD)

clean: cleandeps
	@rm -rf $(BUILD)/* || true

deps:
	mkdir -p ~/.rust/lib
	cd deps/rust-postgres && ./configure
	$(MAKE) -C deps/rust-postgres
	$(MAKE) install -C deps/rust-postgres INSTALL_DIR=$(LIBDIR)

lib: $(BUILD) $(LIBSRC)
	$(RUSTC) -L $(LIBDIR) --out-dir $(BUILD) $(LIBSRC)

test: lib
	@$(RUSTC) -L $(LIBDIR) --test -o $(BUILD)/test $(LIBSRC)
	@$(BUILD)/test $(TEST)

$(LIBDIR):
	mkdir -p $(LIBDIR)

install: $(LIBDIR)
	@cp $(wildcard $(LIB)) $(LIBDIR)

examples: $(BUILD) lib install $(EXAMPLESRCS)
	@mkdir -p $(BUILD)/examples
	@for example in $(EXAMPLESRCS) ; do \
		$(RUSTC) -L $(LIBDIR) $$example --out-dir $(BUILD)/examples || exit 1; \
	done
