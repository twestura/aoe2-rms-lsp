```rms
weather_type PRECIPITATION_STYLE LIVE_COLOR FOG_COLOR WATER_DIRECTION
```

---

Set up precipitation and terrain tinting.
UP only, use [color_correction](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7xt01aajnkw9) and [water_definition](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cyw7be9pc0yt) in DE.

**Arguments**:
- `PRECIPITATION_STYLE`: A number in -4&#8288;&ndash;&#8288;4 (default 0).
  - 0: none, 2: rain, 3: thunderstorm, and 4: snow.
  - Precipitation travels west to east. Use negative values to reverse direction.
- `LIVE_COLOR`: A number in 0&#8288;&ndash;&#8288;255 (default 0). 
  - Terrain tint color in revealed areas.
  - 0 is no tint.
  - 1&#8288;&ndash;&#8288;255 refers to the [color palette](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.wis031bn7kgs).
- `FOG_COLOR`: A number in 0&#8288;&ndash;&#8288;255 (default 0). 
  - Terrain tint color in the fog of war.
  - 0 is no tint.
  - 1&#8288;&ndash;&#8288;255 refers to the [color palette](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.wis031bn7kgs).
- `WATER_DIRECTION`: -1, 0, or 1 (default `0`). Direction of animated water.
  - 0: random.
  - 1: west to east.
  - -1:  east to west.

**Example**: Westward thunderstorm.
```rms
<PLAYER_SETUP>
weather_type -3 16 0 -1
```

**See more**: [weather_type](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ht15fzasksgc)
