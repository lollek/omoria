###############################################################################
# Makefile for imoria
#
# A hopefuly helpful list of things that probably need to be done:
#
# You should fill in the correct paths and stuff in configure.h
# Put a copy of monsters.dat into the DATA_FILE_PATH
# Then pick a uid/gid for the game
# make imoria
# run imoria, and it should create a bunch of files and quit
# make privs
#
###############################################################################
DESTDIR =	/usr/local

CC =		gcc

DEFINES =	-DDO_DEBUG=0
LIBS =		-lncurses -ltermcap -lm -lgdbm

#COPTS =		-Wall -O2 -g3
#COPTS =		-Wall -O2
#COPTS =		-Wall
COPTS =		-Wall -g3

CFLAGS =	$(COPTS) $(DEFINES)
LDFLAGS =	

#
# the owner and group for the game and data files
#
OWNER =	games
GROUP =	games

#
# privs needed on each file depends on if it is for read or write
#
READFILES =	hours.dat moria.dat monsters.dat moria_gcustom.mst
WRITEFILES =	death.log moriamas.dat moriatop.dat moriatrd.dat
DATAFILES =	$(READFILES) $(WRITEFILES)

all:	imoria

CFILES = $(wildcard src/*.c)
OBJFILES = $(addsuffix .o, $(basename $(CFILES)))

.c.o:
	$(CC) $(CFLAGS) -c -o $*.o $*.c

imoria: $(OBJFILES)
	$(CC) $(LDFLAGS) $(OBJFILES) $(LIBS) -o $@
	chown $(OWNER):$(GROUP) imoria
	chmod 2711 imoria
	echo

privs ::
	chown $(OWNER):$(GROUP) imoria $(DATAFILES)
	chmod 2711        imoria
	chmod 640         $(READFILES)
	chmod 660         $(WRITEFILES)
	chmod 755         mhelp.pl

nodata ::
	rm -f hours.dat moria.dat death.log moriamas.dat moriatop.dat moriatrd.dat moria_gcustom.mst TRADE.DUMP

clean ::
	rm -f *.o *~ core imoria i2 Debug.out mm

spotless : nodata clean
