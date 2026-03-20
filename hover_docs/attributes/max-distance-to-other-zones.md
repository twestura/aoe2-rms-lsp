```rms
max_distance_to_other_zones DISTANCE
```

---

Minimum distance in tiles that objects stay away from terrains on which they are restricted from being placed.
Despite the name, this attribute sets a minimum distance rather than a maximum.

- Useful for keeping resources away from coastlines, or deep fish away from beaches.
- If the objects are grouped, distance refers to the center of the group, not individual members.
- Does not apply to road terrains, even though resources cannot be placed on them.
- Has no effect for objects without any terrain restrictions.
- For large values, some objects may not always respect the specified distance.

**Arguments**:
- `DISTANCE`: A number (default 0).

**Example**: Fill the map with gold that avoids being close to water.
```rms
<LAND_GENERATION>
create_land {
  terrain_type WATER
  number_of_tiles 500
  land_position 50 50
}

<OBJECTS_GENERATION>
create_object GOLD {
  number_of_groups 9000
  set_gaia_object_only
  max_distance_to_other_zones 5
}
```

**See more**: [max_distance_to_other_zones](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qf90qwpxyzrs)
