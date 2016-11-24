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
CC =		gcc

CFLAGS =	-Wall -g3 -DDO_DEBUG=0
LDFLAGS =	-lncurses -ltermcap -lm -lgdbm

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
	$(CC) $(LDFLAGS) $(OBJFILES) -o $@

privs ::
	chown $(OWNER):$(GROUP) imoria $(DATAFILES)
	chmod 2711        imoria
	chmod 640         $(READFILES)
	chmod 660         $(WRITEFILES)
	chmod 755         mhelp.pl

nodata ::
	$(RM) data/hours.dat data/moria.dat data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat data/moria_gcustom.mst data/TRADE.DUMP

clean ::
	$(RM) $(OBJFILES) core imoria

spotless : nodata clean
