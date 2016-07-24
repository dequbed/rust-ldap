debug:
	cargo build
	cd protocol; $(MAKE) $(MFLAGS)
	cd client; $(MAKE) $(MFLAGS)

release:
	cargo build --release
	cd protocol; $(MAKE) $(MFLAGS) release
	cd client; $(MAKE) $(MFLAGS) release

clean:
	cargo clean
	cd protocol; $(MAKE) $(MFLAGS) clean
	cd client; $(MAKE) $(MFLAGS) clean
