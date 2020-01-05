# rsfetch - minimalistic fetch program
# See LICENSE file for copyright and license details.

# paths
DBIN = ./target/debug/rsfetch
RBIN = ./target/release/rsfetch
PREFIX = /usr/local

JOBS  ?= 1

# flags
DFLAGS = --color always -j$(JOBS)
RFLAGS = ${DFLAGS} --release

# default arguments
RUN_ARGS = -dehHiklrNcsuU@

# run args
ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

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

bench:
	hyperfine "target/release/rsfetch -NcldkuUH@swp xbps" \
		ufetch pfetch \
		"aura -n \"term\"" \
		"neofetch --disable resolution --disable theme --disable icons --disable term --disable cpu --disable memory" \
		"screenfetch -d \"-gtk;-res;-disk;-mem;-cpu\""

clean:
	rm -rf ./target/
	rm -f rsfetch.tar.gz

install: build
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f rsfetch.1 ${DESTDIR}${PREFIX}/share/man/man1/rsfetch.1
	cp -f ${RBIN} ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/rsfetch

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/rsfetch

.PHONY: all options run clean dist install uninstall bench
