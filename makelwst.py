#!/bin/python3

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
    # simple font table disabled
    0x00,
    # no software-defined color
    0x00,
    # no software-defined position
    0x00,
    # no software-defined font
    0x00,
    # === COLOR TABLE
    # len = 2
    0x02,
    # first color, white
    0xff,
    0xff,
    0xff,
    # second color, yellow
    0xff,
    0xff,
    0x00,
    # === POS TABLE
    # canvas is a 255x255 grid stretched to fit aspect ratio, -127 is radical left, 127 is radical right, 0 is middle
    # len = 1
    0x02,
    # == first position
    # subtitle box anchor point: botmid (topleft = 0x00, topcenter = 0x01, topright, midleft, center, midright, botleft, botmid, botright)
    0x07,
    # attachment reference x: float (float, left of screen, right)
    0x00,
    # attachment point x on grid: middle 
    0x00,
    # attachment reference y: bottom of screen (float, bot, top)
    0x01,
    # distance from reference
    0x16,
    # max box size (0 = unset)
    0x00,
    # == second position
    # anchor point: midleft
    0x03,
    # attachment ref x: left
    0x01,
    # distance from left of screen
    0x18,
    # attachment ref y: float
    0x00,
    # y coord: middle
    0x00,
    # max box size: 1/3 of the screen
    0x55,
    # === FONT TABLE
    # len = 1
    0x01,
    # font
]) + 'SF Pro Text'.encode('utf-8') + bytearray([
    # weight: 500
    0x04,
    # === SUBTITLE TABLE
    # indices for color, pos, font are here because they are not disabled in settings
    # pos index: 0, middle bottom
    0x00,
    # color index: 1, yellow
    0x01,
    # font index: 0
    0x00,
    # time stamp, 100ms in, litte endian
    0x64, 0x00, 0x00, 0x00,
    # subtitle
]) + 'hello lwst\t\n'.encode('utf-8') + bytearray([
    # end of subtitle
    0x00,
    # pos index: 1, right
    0x01,
    # color index: 0, white
    0x01,
    # font index: 0
    0x00,
    # time stamp, 1100ms in, litte endian
    0x4c, 0x04, 0x00, 0x00,
    # subtitle
]) + 'i\'m white!'.encode('utf-8') + bytearray([
    # end of subtitle
    0x00,
])

with open("sub.lwst", "wb") as file:
    file.write(bytes)
