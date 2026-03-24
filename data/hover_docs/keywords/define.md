```rms
#define LABEL
```

---

Defines a label to be used in if statements.

**Arguments**:
- `LABEL`: The name of the label. The name can consist of any nonwhitespace characters, but the convention is to use capital letters, underscores, and digits.

The game predefines labels based on the in-game settings.
See the full list of settings here: [Conditionals](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.vs551a7tyxet)

**Example**: Randomly decide which direction lands spawn.

```rms
start_random
  percent_chance 50 #define TOP_BOTTOM
  percent_chance 50 #define LEFT_RIGHT
end_random

<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type GRASS
  if TOP_BOTTOM
    left_border 20
    right_border 20
  elseif LEFT_RIGHT
    top_border 20
    bottom_border 20
  endif
}
```

**See more**: [#define](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.poaiyxi48mi6)
