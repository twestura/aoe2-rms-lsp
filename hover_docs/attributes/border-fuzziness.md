```rms
border_fuzziness PERCENT
```

---

Specifies the extent to which land growth respects and stops at [borders](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xrncn5cs75or).

- Low values allow lands to exceed borders, giving ragged looking edges when land is constrained by borders.
- 0 causes land growth to ignore borders entirely.
- 100 (or any negative value) means that borders are fully respected, resulting in perfectly straight lands along borders.

**Arguments**:
- `PERCENT`: A number 0&ndash;100 (default 20).

**Example**: A central desert with very fuzzy borders.
```rms
<LAND_GENERATION>
create_land {
  terrain_type DESERT
  land_position 50 50
  land_percent 100
  top_border 40
  right_border 40
  bottom_border 40
  left_border 40
  border_fuzziness 2
}
```

**See more**: [border_fuzziness](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ibpssq2wln80)
