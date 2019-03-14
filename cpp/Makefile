#-----------------------------------------------------------------
# directories
#-----------------------------------------------------------------
BASEDIR = $(shell pwd)
SRCDIR = $(BASEDIR)/src
INCDIR = $(BASEDIR)/include
OBJDIR = $(BASEDIR)/obj
LIBDIR = $(BASEDIR)/lib
BINDIR = $(BASEDIR)/bin

#-----------------------------------------------------------------
# project variables
#-----------------------------------------------------------------
ALL_INCLUDES = $(wildcard $(INCDIR)/*.h)

#-----------------------------------------------------------------
# compiler constants
#-----------------------------------------------------------------
COMPILER = clang++-3.6
OPTIMIZE_FLAGS = -Os -fno-rtti
COMPILER_FLAGS = -c -I$(INCDIR) -std=c++14 -stdlib=libc++ $(OPTIMIZE_FLAGS)
#COMPILER = g++
#COMPILER_FLAGS = -c -I$(INCDIR) -std=c++11

#-----------------------------------------------------------------
# test my lib
#-----------------------------------------------------------------
test: $(BINDIR)/unittest

$(OBJDIR)/unittest.o: $(SRCDIR)/unittest.cpp $(ALL_INCLUDES)
	# create object directories if not already existing
	if [ ! -d "$(OBJDIR)" ]; then mkdir $(OBJDIR); fi
	if [ ! -d "$(LIBDIR)" ]; then mkdir $(LIBDIR); fi
	if [ ! -d "$(BINDIR)" ]; then mkdir $(BINDIR); fi
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(BINDIR)/unittest: $(OBJDIR)/unittest.o $(LIBDIR)/librbf.a
	$(COMPILER) -o$@ $< -L$(LIBDIR) -lrbf -lc++

#-----------------------------------------------------------------
# sandbox
#-----------------------------------------------------------------
sandbox: $(BINDIR)/sandbox

$(OBJDIR)/sandbox.o: $(SRCDIR)/sandbox.cpp
	# create object directories if not already existing
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(BINDIR)/sandbox: $(OBJDIR)/sandbox.o $(LIBDIR)/librbf.a
	$(COMPILER) -o$@ $< -L$(LIBDIR) -lrbf -lc++

#-----------------------------------------------------------------
# library build
#-----------------------------------------------------------------
$(OBJDIR)/element.o: $(SRCDIR)/element.cpp $(INCDIR)/element.h 
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(OBJDIR)/fieldtype.o: $(SRCDIR)/fieldtype.cpp $(INCDIR)/fieldtype.h 
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(OBJDIR)/field.o: $(SRCDIR)/field.cpp $(INCDIR)/field.h 
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(OBJDIR)/record.o: $(SRCDIR)/record.cpp $(INCDIR)/record.h 
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(OBJDIR)/layout.o: $(SRCDIR)/layout.cpp $(INCDIR)/layout.h 
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(OBJDIR)/reader.o: $(SRCDIR)/reader.cpp $(INCDIR)/reader.h $(INCDIR)/layout.h $(INCDIR)/record.h
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

$(OBJDIR)/pugixml.o: $(SRCDIR)/pugixml.cpp $(INCDIR)/pugixml.hpp
	$(COMPILER) $(COMPILER_FLAGS) $< -o$@

rbflib: $(LIBDIR)/librbf.a
#$(LIBDIR)/librbf.a: $(OBJDIR)/element.o $(OBJDIR)/fieldtype.o $(OBJDIR)/field.o $(OBJDIR)/record.o $(OBJDIR)/layout.o $(OBJDIR)/layout.o $(OBJDIR)/pugixml.o
$(LIBDIR)/librbf.a: $(OBJDIR)/element.o $(OBJDIR)/fieldtype.o $(OBJDIR)/field.o $(OBJDIR)/record.o $(OBJDIR)/layout.o $(OBJDIR)/reader.o $(OBJDIR)/pugixml.o 
	ar cr $@ $?

#-----------------------------------------------------------------
# cleanup
#-----------------------------------------------------------------
clean:
	rm lib/*.a
	rm obj/*.o

