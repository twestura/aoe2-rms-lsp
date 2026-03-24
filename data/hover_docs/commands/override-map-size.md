```rms
override_map_size SIDE_LENGTH
```

---

Adjust the square dimensions of the map.

**Arguments**:
- `SIDE_LENGTH`: A number in 36&#8288;&ndash;&#8288;480. Values outside the range are clamped to those limits.

This command is used to map various official water maps one size larger than the size selected in the lobby:

- The command can be used anywhere in the script, but in generation should be used before land generation. The convention is to use it in player setup.
- Affects the scaling of elevation ([set_scale_by_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.8vbd2ko0sw7f) / [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3vkc0lxd4r4a)), terrains ([set_scale_by_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.g4zvtvsbcm29) / [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.cl6w98j0bs9h)), and objects ([set_scaling_to_map_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.ctsq8l5z99u6)).
- Can be used multiple times. Land generation is based on the current size at that point in the script.
- Does not influence the length of the wonder timer, that timer depends only on the size in the lobby.
  - For example, this command can be used to reduce the wonder time by having players choose a tiny size for a script that overrides the size to be larger.

**Example**: Generate a map always at a size of 100x100&nbsp;tiles, discarding lobby settings.

```rms
<PLAYER_SETUP>
override_map_size 100
```

**Example 2**: Generate a map larger than the size specified in the lobby.

```rms
<PLAYER_SETUP>
if TINY_MAP override_map_size 144
elseif SMALL_MAP override_map_size 168
elseif MEDIUM_MAP override_map_size 200
elseif LARGE_MAP override_map_size 220
elseif HUGE_MAP override_map_size 240
elseif GIGANTIC_MAP override_map_size 255
endif
```

**See more**: [override_map_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.sdzu3ermj1ah)
