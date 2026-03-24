```rms
#const IDENTIFIER VALUE
```

---

Defines a constant.

**Arguments**:
- `IDENTIFIER`: The name of the constant. The name can consist of any nonwhitespace characters, but the convention is to use capital letters, underscores, and digits.
- `VALUE`: The value assigned to the constant. Can be an integer or floating point number, another constant, a [random expression](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ml72cdygzrfv), or a [math expression](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bqp5f3wcm40e).

**Example**: Define a mossy road terrain.

```rms
#const ROAD_FUNGUS 39
```

**Example 2**: Define and use variable constants depending on the season.

```rms
start_random
  percent_chance 50 #const TERRAIN_A DESERT
  percent_chance 50 #const TERRAIN_A GRASS
end_random

<LAND_GENERATION>
base_terrain TERRAIN_A
```

**See more**: [#const](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.poaiyxi48mi6)
