# Output directory
BUILD_DIR := build

# Executables
EXECUTABLES := showip

# Source directory
SRC_DIR := .

# Compiler and flags
DEBUG?= -g -O0
CFLAGS?= -O2 -Wall -W -DSDS_ABORT_ON_OOM

# Combine compiler flags
CCOPT= $(CFLAGS)

# Targets
all:  $(addprefix $(BUILD_DIR)/, $(EXECUTABLES))
	@echo "Building" $(EXECUTABLES)

$(BUILD_DIR)/%: $(BUILD_DIR)/%.o
	$(CC) $(CCOPT) $(DEBUG) $(COMPILE_TIME) $< -o $@

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(CC) -c $(CCOPT) $(DEBUG) $(COMPILE_TIME) $< -o $@

# Make build dir if not present
$(BUILD_DIR):
	mkdir -p $@

# List of all dependencies
dep:
	$(CC) -MM *.c

# Clean build and object files
clean:
	rm -rf $(BUILD_DIR)