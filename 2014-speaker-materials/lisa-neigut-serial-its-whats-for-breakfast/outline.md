
# [fit] SERIAL!
# [fit] _It's what's for breakfast_

### lisa neigut **|** @niftynei
#### Animation by Paul Lau & Roy Stanfield

---

```python
import serial

ser = serial.Serial('/dev/tty.usbserial-FTELSI2D', 9600)
ser.write("some stuff")
byte = ser.read()
```

^ so if you're anything like me, this may, or may not
 be the extent of your knowledge of serial
 you've got a file descriptor, and you read and write some stuff to it
 much like a normal file, cept it's called this fancy serial object
 but what is this serial object??

---

![fit](cereal.jpg)

^ its not
Serial as in 'part of a balanced morning'

---

![fit](serialkiller.jpg)

^but maybe Serial as in serial killer.

---

## _serial_ **|** parallel

^ actually yeah, that's kind of a good definition for it. 
 serial is a way of describing a thing that is not parallel
 or things that come one after the other

---

![100%] (parallel_people.jpg)

^ in your computer processor, data gets passed around in blocks
 so like 8 bits will be processed at a time, in a register

---

![fit](8bits.gif)

^ kind of like this one.
 serial, on the other hand is just one bit down a line at a time

---

![200%](single_person.jpg)

^Or one after another
 alright, so how does this work?

---

![150%](pin_out.jpg)

^ here's a classic serial port connector pin connection diagram
these are just connection points between two computers that want
 to talk to each other.
 only some of them are important. i have no idea what a ring indicator is for

---

### _pins, wtf_!

---

    RX      Received Data     Pin 2
    TX      Transmitted Data  Pin 3
    GRD     Ground            Pin 5
    RTS     Ready to Send     Pin 7
    CTS     Clear to Send     Pin 8

^ here's the most important pins
- RX/TX - Data transmission lines
- GRD (Ground) - used for figuring out a base voltage
- RTS/CTS - Control flow signals. 

---

## TX _/_ RX
![200%](single_person.jpg)

^ so that single bit coming down the wire is coming down the TX/RX line

---

![110%](ascii.jpg)

^ since it's one at a time, it ends up being sent as a wave form
A signal is either high or low (theres no middle), since this is hardware, its measured in voltage. 
+12V is a space, or 0, and -12V is a mark, or 1 bit
this particular waveform is for the ascii letter G

---

## GRD
![110%](ascii.jpg)

^ the ground line tells you where zero V is

---

# 00100110

^i've got some data to send
 how do i send it?

---

### _frames_!

---

# [fit] 00100110_|_0_|_11

^so im going to packet that up into a frame -- data, parity (checksum), stop

---

# [fit] _00100110_|0|11
^Frame size (how many bits compose a 'byte' of data)

---

    #define     CS5		    0x00000000	    /* 5 bits (pseudo) */
    #define     CS6		    0x00000100	    /* 6 bits */
    #define     CS7		    0x00000200	    /* 7 bits */
    #define     CS8		    0x00000300	    /* 8 bits */

^ there's 4 different options you can have for frame sizes, 
 it's the number of data bits in a frame

---

# [fit] 00100110|_0_|11

^ parity, data integrity

---

    parity: odd, even, mark, space, none

^Odd, is a count of the number of 1's (**verify)
Even, a count of the number of 0's (**verify)
None, there is no parity bit
 this is usually off

---

# [fit] 00100110|0|_11_

---

    stop: 1, 1.5, 2

^Stop bits: Number of bits that delimit the end of a 'frame'.
Typical options: 1, 1.5 or 2

---

#  **8N1**
#  **5E2**

^ so you can write all this info short hand as data bits, parity bit, number of stop bits
cool. we know what were sending down the wire. how fast are we sending it?

---

### _speed_!

^ serial has a speed. also referred to as the

---

## _**baud rate**_

^ the technical definition for this is the maximum oscillation rate of an electronic signal

---

# [fit] _9600_ bps
# [fit] 115200 bps

^ its Measured in the number of bits per second.
9600 is standard. 115200 is kind of the upper limit

---

```python
import serial

ser = serial.Serial('/dev/tty.usbserial-FTELSI2D', 9600)
ser.write("some stuff")
byte = ser.read()
```

^ so that's what that 9600 was

---

![](heartsStill.jpg)

^ so i'm sending a letter to my granola supplier about how awesome 
their granola is
 the modem has a buffer of about 8kb, thats like 64,000 bits
 the fast connection will fill that up in a little over .5 seconds
 the normal connection's going to take about 7 seconds to empty it

---

![](heartMessage.gif)

^ so as the data gets sent, the buffer gets full, and packets
 start dropping like flies
let's see what all of my love letter actually came through.

---

## <3 <3 <3

--- 

##  33<

^ my granola suppliers a little confused.  33 is less than ... 42?
 it'd be nice if there was some way the modem
 could signal that its full.

--- 

### _control flow_!

---

## RTS _/_ CTS
![180%](stopLight.jpeg)

^ and that's what the last two pins are for

---

    RTS     Ready to Send     Pin 7
    CTS     Clear to Send     Pin 8

^ ready to send -- ive got data
 clear to send -- for the love of god stop
 this is hardware flow control, you can also do this through the
 data lines, tx/rx, using software flow control

---

##   XON   _**17**_
##   XOFF  _**18**_

^ where you pass these special ASCII characters through the data lines instead

---

### _but how_!

---

```
/*
 * Defaults on "first" open.
 */
#define	TTYDEF_IFLAG	(BRKINT	| ICRNL	| IMAXBEL | IXON | IXANY)
#define TTYDEF_OFLAG	(OPOST | ONLCR)
#define TTYDEF_LFLAG	(ECHO | ICANON | ISIG | IEXTEN | ECHOE|ECHOKE|ECHOCTL)
#define	TTYDEF_CFLAG	(CREAD | CS8 | HUPCL)
#define TTYDEF_SPEED	(B9600)
```

^ so all of these options are basically configured as 
OR'd together hex values.
here's the default settings for my mac

---

    /usr/include/sys/termios.h
    /usr/include/sys/ttydefaults.h

^in fact all the data structs and functions for interacting with
 serial are in the termios file, or you can just man termios
 as i was digging through these, i came to realize something
 that serial is  more than just this obsolete protocol for 
 communicating with microprocessors
  
---

![150%](pin_out.jpg)

^ in fact, i use it every day. you could almost say it's as integral to my life as a bowl of cereal 
 in the morning
 it turns out that the process of taking one ASCII character
 and communicating that into parallel for a microprocessor
 or just piping it to another file

---

![fit](terminal.jpg)

^ is useful

---

![fit](terminal.jpg)
![](cereal.jpg)

^ mmmm. tasty serial
