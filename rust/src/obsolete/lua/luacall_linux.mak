#-------------------------------------------------------------------------
# makefile for building the luacall library for Linux
#-------------------------------------------------------------------------

# check if MAKE_DIR variable exist
ifeq ($(LUA_DIR),)
    LUA_DIR = $(shell pwd)
endif

# sets variables
incdir = $(LUA_DIR)
srcdir = $(LUA_DIR)
objdir = $(LUA_DIR)
libdir = $(LUA_DIR)

$(libdir)/libluacall.a: $(objdir)/luacall.o
	ar crv $@ $<

$(objdir)/luacall.o: $(srcdir)/luacall.c
	cc -Wall -I$(incdir) -fPIC -o $@ -c $<

#c2lua: c2lua.c libluacall.a
#	cc -Wall -I$(incdir) -L$(libdir) -o c2lua c2lua.c -lluacall -llua -lm -ldl

clean:
	rm $(objdir)/luacall.o
	rm $(libdir)/libluacall.a

