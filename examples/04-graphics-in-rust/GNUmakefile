ROM_NAME := graphics-in-rust
TARGET_DIR := target/mips-nintendo64-none/release
CARGO_N64_ROM := $(TARGET_DIR)/$(ROM_NAME).n64
FS_BIN := $(TARGET_DIR)/fs.bin
FINAL_ROM := $(ROM_NAME).n64
IPL3 := ../../common/ipl3.bin

all: $(FINAL_ROM)

$(CARGO_N64_ROM): $(shell find src -type f)
	@echo Building $@...
	@cargo n64 build --ipl3 $(IPL3)

$(FS_BIN): $(shell find fs -type f)
	@echo Building $@...
	@cd fs; cat index.txt | xargs cat > ../$(FS_BIN)

$(FINAL_ROM): $(CARGO_N64_ROM) $(FS_BIN)
	@echo Finalising $@...
	@cat $(CARGO_N64_ROM) $(FS_BIN) > $(FINAL_ROM)
	@rs64romtool chksum $@ $@

.PHONY: clean
clean:
	@rm -rf target $(FINAL_ROM)
