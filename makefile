# rsfetch - minimalistic fetch program
# See LICENSE file for copyright and license details.
# This Makefile is proudly stolen from XST project

# paths
DBIN = ./target/debug/rsfetch
RBIN = ./target/release/rsfetch
PREFIX = /usr/local

JOBS  ?= 1

# flags
DFLAGS = --color always -j$(JOBS)
RFLAGS = ${DFLAGS} --release

# default arguments
RUN_ARGS = -deskUH

# run args
ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

# compiler and linker
CC = cargo

all: options debug run

options:
	@echo "CC        = ${CC}"
	@echo "DFLAGS    = ${DFLAGS}"
	@echo "RFLAGS    = ${RFLAGS}"
	@echo "RUNARGS   = ${RUN_ARGS}\n\n"

debug:
	${CC} build ${DFLAGS}

run:
	${DBIN} ${RUN_ARGS}

build:
	${CC} build ${RFLAGS}
	strip --strip-debug $(RBIN)

clean:
	rm -rf ./target/
	rm -f rsfetch.tar.gz

install: build
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f ${RBIN} ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/rsfetch

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/rsfetch

.PHONY: all options run clean dist install uninstall
