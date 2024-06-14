# Executables
EXECUTABLES := showip sockaddr_align server

# Source directory
SRC_DIR := .

# Compiler and flags
DEBUG?= -g -O0
CFLAGS?= -O2 -Wall -W -DSDS_ABORT_ON_OOM

# Combine compiler flags
CCOPT= $(CFLAGS)

# Targets
all:  $(EXECUTABLES)
	@echo "Building" $(EXECUTABLES)

# Dependencies
showip: showip.o
sockaddr_align: sockaddr_align.o
server: server.o

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