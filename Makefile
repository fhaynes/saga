clean:
	$(MAKE) -C main clean

release: clean
	$(MAKE) -C main release

debug: clean
	$(MAKE) -C main debug

install:
	mv main/binaries/saga /usr/local/bin