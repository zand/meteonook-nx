.PHONY: all clean

all:
	cargo build --release
	@rm -rf $(CURDIR)/lib
	@mkdir $(CURDIR)/lib
	@cp $(CURDIR)/target/aarch64-none-elf/release/libmeteonook.a $(CURDIR)/lib/libmeteonook.a
	@rm -rf $(CURDIR)/include
	@mkdir $(CURDIR)/include
	@cp $(CURDIR)/target/aarch64-none-elf/release/meteonook.h $(CURDIR)/include/meteonook.h

clean:
	@rm -rf $(CURDIR)/lib
	@rm -rf $(CURDIR)/include
	cargo clean
