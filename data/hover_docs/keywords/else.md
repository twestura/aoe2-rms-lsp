```rms
if CONDITION
  CODE
elseif CONDITION
  CODE
else
  CODE
endif
```

---

Include a block of code if a condition holds.

- `if` is required to start the block.
- `elseif` is optional and may be included any number of times.
- `else` is optional.
- `endif` is required to end the block.

The conditions are checked in order from top to bottom.
The if statement uses the code block of the first valid condition.
All other blocks are skipped.

**Arguments**:
- `CONDITION`: The condition may be either a label or a constant. The block is used if the label or constant is defined, and the block is skipped otherwise.
- `CODE`: Arbitrary map script code to be included if the corresponding condition exists.

The game predefines labels based on the in-game settings.
See the full list of settings here: [Conditionals](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.vs551a7tyxet)

**Example**: Place a King for the Regicide game mode.

```rms
<OBJECTS_GENERATION>
if REGICIDE
  create_object KING {
    set_place_for_every_player
    min_distance_to_players 7
    max_distance_to_players 9
  }
endif
```

**Example 2**: Place Boars when the game mode is **not** Death Match.

```rms
<OBJECTS_GENERATION>
if DEATH_MATCH
else
  create_object BOAR {
    set_place_for_every_player
    set_gaia_object_only
    min_distance_to_players 16
    max_distance_to_players 22
  }
endif
```

**Example 3**: Manually set the number of Relics based on the map size.

```rms
if TINY_MAP
  #const NUM_RELICS 5
elseif SMALL_MAP
  #const NUM_RELICS 5
elseif MEDIUM_MAP
  #const NUM_RELICS 7
else
  #const NUM_RELICS 11 /* All larger map sizes. */
endif

<OBJECTS_GENERATION>
create_object RELIC { number_of_objects NUM_RELICS }
```

**See more**: [else](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.w2egae5vfwo0)
