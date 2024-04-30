# Project Geographical

A `TUI` (Terminal User Interface) written in RUST for level editing/map creation.

Part of the `WORLD` generation system for the `ZORK/GTAEFk23` engine.

This particular utility is for simple map generation and creates a 2d Array
that can then be fed into the `pray_engine` as part of `GodBabble` (it's actually `.yml`)
and then used to generate the configs and types that we can dump straight into a `MUD` project
and compile down to a deployable AW.

In terms of mapping we use a few chars to represent areas:

* `.` - a null space, essentially just grid filler
* `p` - a path
* `-` - a door leading east
* `-` - a door leading west
* `|` - a door north
* `|` - a door south
* `o` - an area/room/cave etc i.e. a place with stuff that you can move through.

This "map" being bounded and 2d allows for a later config file to reference a grid ref and then 
allow for adding of objects, descriptions etc.

We probably only go to 16*16 in terms of max map size. probably. 

Probable base sizes are `4*4` `8*8`, and `16*16`. 

This might actually be done in this utility albeit its not needed and of course this can all be 
accomplished by just writing the correct format `GodBabble`. 

Go check out [TheOrugginTrail](https://github.com/ArchetypalTech/TheOrugginTrail). 

That's really good and is the `ZORK/GTAEFk23` engine at its core.