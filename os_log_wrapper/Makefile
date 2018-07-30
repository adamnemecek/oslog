CC=clang
CFLAGS=-I$(IDIR)

IDIR=include

_DEPS = os_log_wrapper.h
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

ODIR=obj

_OBJ = os_log_wrapper.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))

SDIR=src

_SRC = os_log_wrapper.c
SRC = $(patsubst %,$(SDIR)/%,$(_SRC))

OUTDIR=out

$(ODIR)/%.o: $(SDIR)/%.c $(DEPS)
	@echo "[Compile]" $<
	@$(CC) -c -o $@ $< $(CFLAGS)

$(OUTDIR)/liboslogwrapper.a: $(OBJ)
	@echo "[Link (Static)]" $<
	@mkdir -p $(OUTDIR)
	@ar rcs $@ $^

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ $(INCDIR)/*~
	rm -rf out

all: liboslogwrapper.a
