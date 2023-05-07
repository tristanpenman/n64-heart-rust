ROM_NAME := booting-rust
TARGET_DIR := target/mips-nintendo64-none/release
CARGO_N64_ROM := $(TARGET_DIR)/$(ROM_NAME).n64
FINAL_ROM := $(ROM_NAME).n64
IPL3 := ../../common/ipl3.bin

all: $(FINAL_ROM)

$(CARGO_N64_ROM): $(shell find src -type f)
	@echo Building $@...
	@cargo n64 build --ipl3 $(IPL3)

$(FINAL_ROM): $(CARGO_N64_ROM)
	@echo Copying $@...
	@cp $(CARGO_N64_ROM) $(FINAL_ROM)

.PHONY: clean
clean:
	$(RM) -r target $(FINAL_ROM)
