```rms
avoid_other_land_zones DISTANCE
```

---

Restricts an object to tiles belonging to its referenced land, and avoids the edges of that land by the specified distance.

- Requires [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) or [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv).
- Even if 0 or a negative value is specified, this attribute still forces objects to stay on the land. Do not use this attribute at all if you want objects to be placed beyond the borders of the land.

**Arguments**:
- `DISTANCE`: A number (default: no avoidance).

**Example**: Place gold on a specific desert land, keeping it away from the edges.
```rms
<LAND_GENERATION>
create_land {
  terrain_type DESERT
  land_percent 10
  land_id 1
}

<OBJECTS_GENERATION>
create_object GOLD {
  place_on_specific_land_id 1
  set_gaia_object_only
  number_of_objects 999
  avoid_other_land_zones 4
}
```

**See more**: [avoid_other_land_zones](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.axswyohsolzw)
