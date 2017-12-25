CARGO = cargo

test: 
	@$(CARGO) test

release:
	@$(CARGO) build --release
	mv target/release/main binaries/saga
	chmod ugo+x binaries/main

debug:
	@$(CARGO) build
	mv target/debug/main binaries/saga
	chmod ugo+x binaries/saga
	mv binaries/saga /usr/local/bin

clean:
	@$(CARGO) clean

install:
	mv binaries/saga /usr/local/bin
