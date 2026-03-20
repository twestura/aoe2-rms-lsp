```rms
terrain_mask LAYER
```

---

Enables terrain masking, layering the created terrain visually over or under the base terrain.

The terrain inherits all properties, placement restrictions, automatic objects (such as trees for forest terrains), minimap color, and other characteristics from the terrain underneath.

- Masking layers 1 and 2 have different visual masking patterns.
- Terrain has animated water if any of the component terrains are water.
- Legacy terrains that are already a blend of two texture files cannot be masked visually. They contribute fully to the final terrain appearance. These terrains are: `GRASS_SNOW`, `DIRT_SNOW`, `DLC_MOORLAND`, `DLC_JUNGLELEAVES`, `DLC_DRYROAD`, `DLC_JUNGLEROAD`, `DLC_ROADGRAVEL`, road snow (38), and road fungus (39).
- There are special cases with beach terrains which may not always mask as expected.

**Arguments**:
- `LAYER`: Whether to mask the terrain over or under (default 0, no masking).
  - 1 to mask the new terrain over [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ptxp1ht2fh0p) (new terrain is visual only, inherits base terrain properties).
  - 2 to mask the new terrain under [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ptxp1ht2fh0p) (new terrain provides properties, base terrain is visual only).

**Example**: Snow masked on top of grass, producing grass decoration objects.
```rms
<TERRAIN_GENERATION>
create_terrain SNOW {
  base_terrain GRASS
  land_percent 50
  terrain_mask 1
}
```

**See more**: [terrain_mask](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.e0ug99qovffm)
