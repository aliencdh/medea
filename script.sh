#!/bin/bash
for w in $(bspc query -N -n .window); do printf '%s - %s\n' "$(bspc query -D -n $w --names)" "$(xtitle $w)"; done
