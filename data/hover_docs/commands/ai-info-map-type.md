```rms
ai_info_map_type MAP_TYPE IS_NOMAD IS_MICHI SHOW_TYPE
```

---

Provides information about the map to AIs.

**Arguments**:
- `MAP_TYPE`: A [map type](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.jxjsnahvu5u4) constant (default `CUSTOM`).
- `IS_NOMAD`: `1` if the map is nomad style (no starting Town Center), otherwise `0` (default `0`).
- `IS_MICHI`: `1` if the map is Michi style (forest completely separating players), otherwise `0` (default `0`).
- `SHOW_TYPE`: `1` to show the chosen map type in the objectives window (default `0`), does not work in DE.

If the map is not similar to an existing map type, omit this command or use `CUSTOM` as the map type.

**Example**: A modified Arabia map.
```rms
<PLAYER_SETUP>
ai_info_map_type ARABIA 0 0 0
```

**See more**: [ai_info_map_type](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6o9sfjx8tww0)
