default: build

build:
	cargo build --release

install:
	mkdir -p ${DESTDIR}/usr/bin
	cp target/release/vex ${DESTDIR}/usr/bin/vex

clean:
	rm Cargo.lock
	rm -r target
