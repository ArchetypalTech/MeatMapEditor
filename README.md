# Project Geographiser

A `TUI` (Terminal User Interface) written in RUST for level editing/map creation.

Part of the `WORLD` generation system for the `ZORK/GTAEFk23` engine.

This particular utility is for simple map generation and creates a 2d Array
that can then be fed into the `pray_engine` as part of `GodBabble` (it's actually `.yml`)
and then used to generate the configs and types that we can dump straight into a `MUD` project
and compile down to a deployable AW.

Ovbs one could just also feed the string representation into the later tooling but... :shrugs: wtf.

Go check out [TheOrugginTrail](https://github.com/ArchetypalTech/TheOrugginTrail). 

That's really good and is the `ZORK/GTAEFk23` engine at its core.