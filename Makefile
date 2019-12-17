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
RFLAGS = ${DFLAGS} --release -j$(JOBS)

# run args
ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

# compiler and linker
CC = cargo

all: options run

opt:
	@echo :: OPTIONS
	@echo "CC        = ${CC}"
	@echo "DFLAGS    = ${DFLAGS}"
	@echo "RFLAGS    = ${RFLAGS}"
	@echo "RUNARGS   = ${RUN_ARGS}"

debug:
	${CC} build ${DFLAGS}

run: debug
	${DBIN} ${RUN_ARGS}

build:
	${CC} build ${RFLAGS}

clean:
	rm -rf ./target/
	rm -f rsfetch.tar.gz

dist: options clean build
	mkdir -p rsfetch-tmp
	cp -f ${RBIN} ./rsfetch-tmp/ 
	tar -cf rsfetch.tar rsfetch-tmp
	gzip rsfetch.tar
	rm -rf rsfetch

install: build
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f ${RBIN} ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/rsfetch

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/rsfetch

.PHONY: all options run clean dist install uninstall
