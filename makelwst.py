#!/bin/python3

# i should rethink the timestamps, definitely add an end timestamp and make it so a duration of 0 means til the next subtitle appears

bytes = bytearray([
    # lwst
    0x6c, 0x77, 0x73, 0x74,
    # version 1
    0x01,
    # these below could be just one byte but whatever
    # encoding: 0 = utf8, 1 = ascii
    0x00,
    # no compression
    0x00,
    # simple font table disabled (names + weight instead of indices for mcu)
    0x00,
    # no software-defined position
    0x00,
    # no software-defined color
    0x00,
    # no software-defined font
    0x00,
    # === COLOR TABLE
    # len = 2
    0x02,
    # == first color, white
    0xff,
    0xff,
    0xff,
    # == second color, yellow
    0xff,
    0xff,
    0x00,
    # === POS TABLE
    # canvas is a 255x255 grid stretched to fit aspect ratio, 0 is radical left, 255 is radical right (127 and 128 is middle)
    #                                                         0 is bottom, 255 is top (127 and 128 is middle)
    # len = 2
    0x02,
    # == first position
    # subtitle box anchor point: botcenter (topleft = 0x00, topcenter = 0x01, topright, midleft, center, midright, botleft, botcenter, botright)
    0x07,
    # attachment point x on grid: middle 
    0x00,
    # attachment point y on grid: a bit to the top from the bottom
    0x16,
    # max box width (0 = unset)
    0x00,
    # == second position
    # anchor point: midleft
    0x03,
    # attachment point on x: 10 so a bit to the right from radical left
    0x10,
    # attachment point on y: middle
    127,
    # max box width: 1/3 of the screen
    0x55,
    # === FONT TABLE
    # len = 1
    0x01,
    # font
]) + 'SF Pro Text'.encode('utf-8') + bytearray([
    # font name end
    0x00,
    # weight: 500
    0x04,
    # === SUBTITLE TABLE
    # == subtitle 1
]) + 'hello lwst'.encode('utf-8') + bytearray([
    # end of subtitle 1
    0x00,
    # == subtitle 2
]) + 'i\'m white!'.encode('utf-8') + bytearray([
    # end of subtitle 2
    0x00,
    # == end of subtitle table
    0x00,
    # === TIMING ARRAY
    # indices for color, pos, font are here because they are not disabled in settings
    # == timing entry 1
    # pos index: 0, middle bottom
    0x00,
    # color index: 1, yellow
    0x01,
    # font index: 0
    0x00,
    # time stamp, 100ms in, litte endian
    0x64, 0x00, 0x00,
    # subtitle 1, hello lwst
    0x00, 0x00,
    # == timing entry 2
    # pos index: 1, right
    0x01,
    # color index: 0, white
    0x01,
    # font index: 0
    0x00,
    # time stamp, 1100ms in, litte endian
    0x4c, 0x04, 0x00,
    # subtitle 2, i'm white
    0x01, 0x00,
])

with open("sub.lwst", "wb") as file:
    file.write(bytes)
