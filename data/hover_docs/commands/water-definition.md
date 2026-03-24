```rms
water_definition WATER_TYPE
```

---

Specify a water profile for the map.

DE only, use [weather_type](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ht15fzasksgc) in UP.

**Arguments**:
- `WATER_TYPE`: A [water definition constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.71e8lxz43czw) (default 0, which is `WD_DEFAULT`).
  - None of the water definition constants are predefined. Define the constant with `#const` before use. The list of constants is 0-indexed.

Each option applies a different color correction, wave style, and reflections on 3D water.

- Not visible if the "Render 3D Water" setting is disabled.
- Waves on shorelines are controlled separately by [enable_waves](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fno7myp7722j).
- General color correction is controlled separately by [color_correction]([URL](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7xt01aajnkw9)).

**Example**: Water with rapid waves and limited sunlight.
```rms
#const WD_HURRICANE 8
<PLAYER_SETUP>
water_definition WD_HURRICANE
```

**See more**: [water_definition](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cyw7be9pc0yt)
