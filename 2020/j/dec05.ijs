#! /usr/bin/env ijconsole

NB. Seats are binary numbers with B and R as ones.
seats =: #. 'BR' e.~ > cutopen 1!:1 <'input/dec05.txt'
echo 'part 1'
echo >./ seats
echo 'part 2'
NB. Sort the seats, then the difference between pairs of seats.
NB. The missing seat is: the point where we see a difference of 2, plus the 
NB. starting seat, plus one.
echo <./ seats + 1 + 2 i.~ 2 -~/\ /:~ seats
exit 0
