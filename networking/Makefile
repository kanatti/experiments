# Executables
EXECUTABLES := showip

# Source directory
SRC_DIR := .

# Compiler and flags
DEBUG?= -g -O0
CFLAGS?= -O2 -Wall -W -DSDS_ABORT_ON_OOM

# Combine compiler flags
CCOPT= $(CFLAGS)

# Dependencies
showip: showip.o

# Targets
all:  $(EXECUTABLES)
	@echo "Building" $(EXECUTABLES)

$(SRC_DIR)/%: $(SRC_DIR)/%.o
	$(CC) $(CCOPT) $(DEBUG) $(COMPILE_TIME) $< -o $@

$(SRC_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) -c $(CCOPT) $(DEBUG) $(COMPILE_TIME) $< -o $@

# List of all dependencies
dep:
	$(CC) -MM *.c

# Clean build and object files
clean:
	rm -rf $(EXECUTABLES) *.o