```rms
top_border PERCENT
right_border PERCENT
bottom_border PERCENT
left_border PERCENT
```

---

Specifies a percentage of map width that land placement and growth must stay away from a given border.

- **Top** is **northwest**; **right** is **northeast**; **bottom** is **southeast**; **left** is **southwest**.
- A hard-coded feature makes lands look like octagons instead of squares when constrained by borders.
- Borders shift the entire circle of all player lands.
- Multiple rings of player lands with different borders cannot be created — all player lands will be in the same circle.
- Due to rounding, the exact number of tiles that a given percentage corresponds to may differ per side.
- Negative values can be used as long as the land origin stays inside the map. To ensure this, either specify a [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fnezpf6c85yf) within the map, or specify a sufficiently large [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh).
- **Bug**: asymmetric borders for player lands can cause issues when `top_border` is larger than other borders. Avoid by always pairing `top_border` with at least one other border attribute when creating player lands. See more: [RMS Border Bugs Exposed](http://aok.heavengames.com/cgi-bin/forums/display.cgi?action=ct&f=28,42496,0,365).

**Arguments**:
- `PERCENT`: A number 0&ndash;99 (default 0).

**Example**: Place all players in the top corner of the map.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DIRT
  land_percent 100
  top_border 0
  right_border 0
  bottom_border 60
  left_border 60
}
```

**See more**: [top_border](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7oog42u9uwjn), [right_border](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h0hz0dquv1vc), [bottom_border](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mwha0mo8ve9s), [left_border](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xrncn5cs75or)
