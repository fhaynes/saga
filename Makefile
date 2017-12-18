clean:
	$(MAKE) -C main clean

release: clean
	$(MAKE) -C main release

debug: clean
	$(MAKE) -C main debug

test:
	$(MAKE) -C main test
	$(MAKE) -C web test
	$(MAKE) -C inverted_index test

install:
	mv main/binaries/saga /usr/local/bin