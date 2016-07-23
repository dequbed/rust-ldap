debug:
	cargo build
	cd protocol; $(MAKE) $(MFLAGS)

release:
	cargo build --release
	cd protocol; $(MAKE) $(MFLAGS) release

clean:
	cargo clean
	cd protocol; $(MAKE) $(MFLAGS) clean
