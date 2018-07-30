#
#  Makefile
#  liboslog
#
#  Created by Søren Mortensen on 29/07/2018.
#  Copyright © 2018 Søren Mortensen. All rights reserved.
#

CC=clang
CFLAGS=-I$(INC_DIR)

INC_DIR=include

_DEPS = oslog.h
DEPS = $(patsubst %,$(INC_DIR)/%,$(_DEPS))

OBJ_DIR=obj

_OBJ = oslog.o
OBJ = $(patsubst %,$(OBJ_DIR)/%,$(_OBJ))

SRC_DIR=src

_SRC = oslog.c
SRC = $(patsubst %,$(SRC_DIR)/%,$(_SRC))

OUT_DIR=out

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c $(DEPS)
	@echo "[Compile]" $<
	@mkdir -p $(OBJ_DIR)
	@$(CC) -c -o $@ $< $(CFLAGS)

$(OUT_DIR)/liboslog.a: $(OBJ)
	@echo "[Link (Static)]" $<
	@mkdir -p $(OUT_DIR)
	@ar rcs $@ $^

$(OUT_DIR):
	@mkdir -p $(OUT_DIR)

.PHONY: clean $(OBJ_DIR) $(OUT_DIR)

clean:
	rm -f $(OBJ_DIR)/*.o $(OUT_DIR)/*.a

all: liboslog.a
