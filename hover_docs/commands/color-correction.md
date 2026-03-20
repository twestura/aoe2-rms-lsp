```rms
color_correction COLOR_CORRECTION
```

---

Applies a color correction profile to the map lighting.

- Not visible if the "Map Lighting" setting is disabled.
- Water-specific color correction is controlled by [water_definition](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cyw7be9pc0yt).

**Arguments**:
- `COLOR_CORRECTION`: A [color correction constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.ptggu3c4a8jy) (default: no color correction).
  - `CC_DEFAULT`, `CC_AUTUMN`, `CC_WINTER`, `CC_JUNGLE`, `CC_DESERT`, `CC_NIGHT`, `CC_EVENING`, `CC_SPRING`, `CC_SAVANNAH`, `CC_ARCTIC`, `CC_RAINFOREST`, `CC_SWAMP`, `CC_STEPPES`, `CC_MISTY`, `CC_SUMMER`, `CC_MURKY`, `CC_BRUMOUS`, `CC_TWILIGHT`, `CC_DARKNESS`

**Example**: Desert-themed lighting.
```rms
<TERRAIN_GENERATION>
color_correction CC_DESERT
```

**See more**: [color_correction](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7xt01aajnkw9)
