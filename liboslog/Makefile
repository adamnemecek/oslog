CC=clang
CFLAGS=-I$(IDIR)

IDIR=include

_DEPS = oslog.h
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

ODIR=obj

_OBJ = oslog.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))

SDIR=src

_SRC = oslog.c
SRC = $(patsubst %,$(SDIR)/%,$(_SRC))

OUTDIR=out

$(ODIR)/%.o: $(SDIR)/%.c $(DEPS)
	@echo "[Compile]" $<
	@$(CC) -c -o $@ $< $(CFLAGS)

$(OUTDIR)/liboslog.a: $(OBJ)
	@echo "[Link (Static)]" $<
	@mkdir -p $(OUTDIR)
	@ar rcs $@ $^

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ $(INCDIR)/*~
	rm -rf out

all: liboslog.a
