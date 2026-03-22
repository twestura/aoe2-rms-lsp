//! Information about Local Instructions: Commands and Arguments.
//!
//! Note the original Random Map Scripting Guide included with the Conquerors
//! refers to "Global Instructions" for if blocks and define/const statements,
//! and "Local Instructions" for commands and attributes.

/// The maximum number of arguments for any instruction.
pub const MAX_ARGS: usize = 4;

/// The argument specification for a command or attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Arguments {
    /// The number of required arguments.
    pub required: usize,
    /// The number of optional arguments.
    pub optional: usize,
}

impl Arguments {
    /// Returns the total number of arguments, required and optional.
    pub fn total(&self) -> usize {
        self.required + self.optional
    }
}

/// Returns `true` if the given string `s` is a section header,
/// `false` otherwise.
pub fn is_section_header(s: &str) -> bool {
    matches!(
        s,
        "<PLAYER_SETUP>"
            | "<LAND_GENERATION>"
            | "<ELEVATION_GENERATION>"
            | "<CLIFF_GENERATION>"
            | "<TERRAIN_GENERATION>"
            | "<CONNECTION_GENERATION>"
            | "<OBJECTS_GENERATION>"
    )
}

/// Returns the argument specification for the given token.
pub fn arguments(token: &str) -> Arguments {
    let args = |required, optional| Arguments { required, optional };
    match token {
        // Commands — PLAYER_SETUP
        "behavior_version" => args(1, 0),
        "override_map_size" => args(1, 0),
        "set_gaia_civilization" => args(1, 0),
        "ai_info_map_type" => args(4, 0),
        "effect_amount" => args(4, 0),
        "effect_percent" => args(4, 0),
        "guard_state" => args(4, 0),
        "terrain_state" => args(4, 0),
        "weather_type" => args(4, 0),
        "water_definition" => args(1, 0),
        // Commands — LAND_GENERATION
        "enable_waves" => args(1, 0),
        "create_elevation" => args(1, 0),
        // Commands — TERRAIN_GENERATION
        "color_correction" => args(1, 0),
        "create_terrain" => args(1, 0),
        // Commands — CLIFF_GENERATION
        "cliff_type" => args(1, 0),
        "min_number_of_cliffs" => args(1, 0),
        "max_number_of_cliffs" => args(1, 0),
        "min_length_of_cliff" => args(1, 0),
        "max_length_of_cliff" => args(1, 0),
        "cliff_curliness" => args(1, 0),
        "min_distance_cliffs" => args(1, 0),
        "min_terrain_distance" => args(1, 0),
        // Commands — CONNECTION_GENERATION
        "create_connect_land_zones" => args(2, 0),
        // Commands — OBJECTS_GENERATION
        "create_actor_area" => args(4, 0),
        "create_object_group" => args(1, 0),
        "create_object" => args(1, 0),
        // Attributes — land
        "terrain_type" => args(1, 0),
        "land_percent" => args(1, 0),
        "number_of_tiles" => args(1, 0),
        "base_size" => args(1, 0),
        "generate_mode" => args(1, 0),
        "land_position" => args(2, 0),
        "circle_radius" => args(1, 1),
        "left_border" => args(1, 0),
        "right_border" => args(1, 0),
        "top_border" => args(1, 0),
        "bottom_border" => args(1, 0),
        "border_fuzziness" => args(1, 0),
        "clumping_factor" => args(1, 0),
        "land_conformity" => args(1, 0),
        "base_elevation" => args(1, 0),
        "assign_to_player" => args(1, 0),
        "assign_to" => args(4, 0),
        "zone" => args(1, 0),
        "other_zone_avoidance_distance" => args(1, 0),
        "min_placement_distance" => args(1, 0),
        "land_id" => args(1, 0),
        // Attributes — elevation
        "number_of_clumps" => args(1, 0),
        "spacing" => args(1, 0),
        // Attributes — terrain
        "beach_terrain" => args(1, 0),
        "terrain_mask" => args(1, 0),
        "spacing_to_other_terrain_types" => args(1, 0),
        "spacing_to_specific_terrain" => args(2, 0),
        "height_limits" => args(2, 0),
        // Attributes — connection
        "default_terrain_replacement" => args(1, 0),
        "replace_terrain" => args(2, 0),
        "terrain_cost" => args(2, 0),
        "terrain_size" => args(3, 0),
        // Attributes — objects
        "add_object" => args(2, 0),
        "number_of_objects" => args(1, 0),
        "number_of_groups" => args(1, 0),
        "group_variance" => args(1, 0),
        "group_placement_radius" => args(1, 0),
        "min_connected_tiles" => args(1, 0),
        "resource_delta" => args(1, 0),
        "second_object" => args(1, 0),
        "place_on_specific_land_id" => args(1, 0),
        "avoid_other_land_zones" => args(1, 0),
        "min_distance_to_players" => args(1, 0),
        "max_distance_to_players" => args(1, 0),
        "terrain_to_place_on" => args(1, 0),
        "layer_to_place_on" => args(1, 0),
        "max_distance_to_other_zones" => args(1, 0),
        "avoid_forest_zone" => args(0, 1),
        "avoid_cliff_zone" => args(0, 1),
        "min_distance_to_map_edge" => args(1, 0),
        "min_distance_group_placement" => args(1, 0),
        "temp_min_distance_group_placement" => args(1, 0),
        "require_path" => args(1, 0),
        "actor_area" => args(1, 0),
        "actor_area_radius" => args(1, 0),
        "actor_area_to_place_in" => args(1, 0),
        "avoid_actor_area" => args(1, 0),
        "set_facet" => args(1, 0),
        "set_avoid_player_start_areas" => args(0, 1),
        // Command-Attributes
        "base_terrain" => args(1, 0),
        "base_layer" => args(1, 0),
        // Keywords
        "if" => args(1, 0),
        "elseif" => args(1, 0),
        "percent_chance" => args(1, 0),
        "#define" => args(1, 0),
        "#const" => args(2, 0),
        "#include_drs" => args(1, 0),
        "#includexs" => args(1, 0),
        _ => Arguments::default(),
    }
}
