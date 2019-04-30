# fetch - minimalistic fetch program
# See LICENSE file for copyright and license details.
# This Makefile is proudly stolen from XST project

VERSION = 0.2

# paths
DBIN = ./target/debug/fetch
RBIN = ./target/release/fetch
PREFIX = /usr/local

# flags
DFLAGS = --quiet --color always
RFLAGS = ${DFLAGS} --release

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
	@${CC} build ${DFLAGS}

run: debug
	@${DBIN} ${RUN_ARGS}

build:
	@echo :: CARGO BUILD
	@${CC} build ${RFLAGS}

clean:
	@echo :: CLEANING
	@rm -rf ./target/
	@rm -f fetch.tar.gz

dist: options clean build
	@echo :: CREATING TARBALL
	@mkdir -p fetch-tmp
	@cp -f ${RBIN} ./fetch-tmp/ 
	@tar -cf fetch.tar fetch-tmp
	@gzip fetch.tar
	@rm -rf fetch

install: build
	@echo :: INSTALLING TO ${DESTDIR}${PREFIX}/bin
	@mkdir -p ${DESTDIR}${PREFIX}/bin
	@cp -f ${RBIN} ${DESTDIR}${PREFIX}/bin
	@chmod 755 ${DESTDIR}${PREFIX}/bin/fetch

uninstall:
	@echo removing executable file from ${DESTDIR}${PREFIX}/bin
	@rm -f ${DESTDIR}${PREFIX}/bin/fetch
	@echo removing manual page from ${DESTDIR}${MANPREFIX}/man1
	@rm -f ${DESTDIR}${MANPREFIX}/man1/fetch.1

.PHONY: all options run clean dist install uninstall