# Detect OS
ifeq ($(OS),Windows_NT)
    EXE := .exe
    RM := rmdir /S /Q
    MKDIR := mkdir
    RUN := bin\client$(EXE)
    SEP := \\
else
    EXE :=
    RM := rm -rf
    MKDIR := mkdir -p
    RUN := ./bin/client$(EXE)
    SEP := /
endif

SRC := src$(SEP)client$(SEP)*.c
OUT := bin$(SEP)client$(EXE)

client: clean
	gcc $(SRC) -o $(OUT)

test: client
	$(RUN)

clean:
	-$(RM) bin 2>nul || true
	$(MKDIR) bin