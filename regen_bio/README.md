# `regen_bio`

## Goals

* provide a datatype for describing the names of species, providing minimal validation
that the name conforms to binomial nomenclature in a way that is non-ambiguous. To
achieve disambiguation, the kingdom is always includes as name conflicts in the wild
always happen between different kingdoms.

## Non-goals

* provide a vocabulary of all extant species (those exist), although this datatype
could support the management of such a database