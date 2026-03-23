//! Handles completion requests for the language server.

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionResponse, CompletionTextEdit, Documentation,
    InsertTextFormat, MarkupContent, MarkupKind, Position, Range, TextEdit,
};

use crate::{
    rms::{self, CompletionText},
    server::lookup_hover,
};

/// All tokens that are recognized by the language server and can be offered as
/// completion suggestions, along with their lowercase forms for case-insensitive
/// matching, their kinds for coloring and icons in the completion popup, and
/// optional snippets for expanding to full syntax with placeholders.
#[rustfmt::skip]
static COMPLETABLE_TOKENS: &[CompletableToken] = &[
    // Sections
    CompletableToken { label: "<PLAYER_SETUP>", lower: "<player_setup>", kind: CompletionItemKind::MODULE, snippet: None },
    CompletableToken { label: "<LAND_GENERATION>", lower: "<land_generation>", kind: CompletionItemKind::MODULE, snippet: None },
    CompletableToken { label: "<ELEVATION_GENERATION>", lower: "<elevation_generation>", kind: CompletionItemKind::MODULE, snippet: None },
    CompletableToken { label: "<CLIFF_GENERATION>", lower: "<cliff_generation>", kind: CompletionItemKind::MODULE, snippet: None },
    CompletableToken { label: "<TERRAIN_GENERATION>", lower: "<terrain_generation>", kind: CompletionItemKind::MODULE, snippet: None },
    CompletableToken { label: "<CONNECTION_GENERATION>", lower: "<connection_generation>", kind: CompletionItemKind::MODULE, snippet: None },
    CompletableToken { label: "<OBJECTS_GENERATION>", lower: "<objects_generation>", kind: CompletionItemKind::MODULE, snippet: None },
    // Commands
    CompletableToken { label: "random_placement", lower: "random_placement", kind: CompletionItemKind::FUNCTION, snippet: None },
    CompletableToken { label: "direct_placement", lower: "direct_placement", kind: CompletionItemKind::FUNCTION, snippet: None },
    CompletableToken { label: "grouped_by_team", lower: "grouped_by_team", kind: CompletionItemKind::FUNCTION, snippet: None },
    CompletableToken { label: "nomad_resources", lower: "nomad_resources", kind: CompletionItemKind::FUNCTION, snippet: None },
    CompletableToken { label: "force_nomad_treaty", lower: "force_nomad_treaty", kind: CompletionItemKind::FUNCTION, snippet: None },
    CompletableToken { label: "behavior_version", lower: "behavior_version", kind: CompletionItemKind::FUNCTION, snippet: Some("behavior_version ${1:VERSION_NUMBER}") },
    CompletableToken { label: "override_map_size", lower: "override_map_size", kind: CompletionItemKind::FUNCTION, snippet: Some("override_map_size ${1:SIDE_LENGTH}") },
    CompletableToken { label: "set_gaia_civilization", lower: "set_gaia_civilization", kind: CompletionItemKind::FUNCTION, snippet: Some("set_gaia_civilization ${1:CIVILIZATION}") },
    CompletableToken { label: "ai_info_map_type", lower: "ai_info_map_type", kind: CompletionItemKind::FUNCTION, snippet: Some("ai_info_map_type ${1:MAP_TYPE} ${2:IS_NOMAD} ${3:IS_MICHI} ${4:SHOW_TYPE}") },
    CompletableToken { label: "effect_amount", lower: "effect_amount", kind: CompletionItemKind::FUNCTION, snippet: Some("effect_amount ${1:EFFECT_TYPE} ${2:TYPE} ${3:ATTRIBUTE_TYPE} ${4:AMOUNT}") },
    CompletableToken { label: "effect_percent", lower: "effect_percent", kind: CompletionItemKind::FUNCTION, snippet: Some("effect_percent ${1:EFFECT_TYPE} ${2:TYPE} ${3:ATTRIBUTE_TYPE} ${4:PERCENT}") },
    CompletableToken { label: "guard_state", lower: "guard_state", kind: CompletionItemKind::FUNCTION, snippet: Some("guard_state ${1:OBJECT_TYPE} ${2:RESOURCE_TYPE} ${3:RESOURCE_DELTA} ${4:FLAGS}") },
    CompletableToken { label: "terrain_state", lower: "terrain_state", kind: CompletionItemKind::FUNCTION, snippet: Some("terrain_state ${1:MODE} ${2:PARAMETER_1} ${3:PARAMETER_2} ${4:FLAGS}") },
    CompletableToken { label: "weather_type", lower: "weather_type", kind: CompletionItemKind::FUNCTION, snippet: Some("weather_type ${1:PRECIPITATION_STYLE} ${2:LIVE_COLOR} ${3:FOG_COLOR} ${4:WATER_DIRECTION}") },
    CompletableToken { label: "water_definition", lower: "water_definition", kind: CompletionItemKind::FUNCTION, snippet: Some("water_definition ${1:WATER_TYPE}") },
    CompletableToken { label: "enable_waves", lower: "enable_waves", kind: CompletionItemKind::FUNCTION, snippet: Some("enable_waves ${1:SHOW_WAVES}") },
    CompletableToken { label: "create_player_lands", lower: "create_player_lands", kind: CompletionItemKind::FUNCTION, snippet: Some("create_player_lands {\n\t$0\n}") },
    CompletableToken { label: "create_land", lower: "create_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_land {\n\t$0\n}") },
    CompletableToken { label: "create_elevation", lower: "create_elevation", kind: CompletionItemKind::FUNCTION, snippet: Some("create_elevation ${1:MAX_HEIGHT} {\n\t$0\n}") },
    CompletableToken { label: "color_correction", lower: "color_correction", kind: CompletionItemKind::FUNCTION, snippet: Some("color_correction ${1:COLOR_CORRECTION}") },
    CompletableToken { label: "create_terrain", lower: "create_terrain", kind: CompletionItemKind::FUNCTION, snippet: Some("create_terrain ${1:TERRAIN} {\n\t$0\n}") },
    CompletableToken { label: "cliff_type", lower: "cliff_type", kind: CompletionItemKind::FUNCTION, snippet: Some("cliff_type ${1:CLIFF_TYPE}") },
    CompletableToken { label: "min_number_of_cliffs", lower: "min_number_of_cliffs", kind: CompletionItemKind::FUNCTION, snippet: Some("min_number_of_cliffs ${1:NUMBER}") },
    CompletableToken { label: "max_number_of_cliffs", lower: "max_number_of_cliffs", kind: CompletionItemKind::FUNCTION, snippet: Some("max_number_of_cliffs ${1:NUMBER}") },
    CompletableToken { label: "min_length_of_cliff", lower: "min_length_of_cliff", kind: CompletionItemKind::FUNCTION, snippet: Some("min_length_of_cliff ${1:LENGTH}") },
    CompletableToken { label: "max_length_of_cliff", lower: "max_length_of_cliff", kind: CompletionItemKind::FUNCTION, snippet: Some("max_length_of_cliff ${1:LENGTH}") },
    CompletableToken { label: "cliff_curliness", lower: "cliff_curliness", kind: CompletionItemKind::FUNCTION, snippet: Some("cliff_curliness ${1:PERCENT}") },
    CompletableToken { label: "min_distance_cliffs", lower: "min_distance_cliffs", kind: CompletionItemKind::FUNCTION, snippet: Some("min_distance_cliffs ${1:DISTANCE}") },
    CompletableToken { label: "min_terrain_distance", lower: "min_terrain_distance", kind: CompletionItemKind::FUNCTION, snippet: Some("min_terrain_distance ${1:DISTANCE}") },
    CompletableToken { label: "accumulate_connections", lower: "accumulate_connections", kind: CompletionItemKind::FUNCTION, snippet: None },
    CompletableToken { label: "create_connect_all_players_land", lower: "create_connect_all_players_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_all_players_land {\n\t$0\n}") },
    CompletableToken { label: "create_connect_teams_lands", lower: "create_connect_teams_lands", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_teams_lands {\n\t$0\n}") },
    CompletableToken { label: "create_connect_all_lands", lower: "create_connect_all_lands", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_all_lands {\n\t$0\n}") },
    CompletableToken { label: "create_connect_same_land_zones", lower: "create_connect_same_land_zones", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_same_land_zones {\n\t$0\n}") },
    CompletableToken { label: "create_connect_land_zones", lower: "create_connect_land_zones", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_land_zones ${1:ZONE_1} ${2:ZONE_2} {\n\t$0\n}") },
    CompletableToken { label: "create_connect_to_nonplayer_land", lower: "create_connect_to_nonplayer_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_to_nonplayer_land {\n\t$0\n}") },
    CompletableToken { label: "create_actor_area", lower: "create_actor_area", kind: CompletionItemKind::FUNCTION, snippet: Some("create_actor_area ${1:X} ${2:Y} ${3:ACTOR_AREA_ID} ${4:RADIUS}") },
    CompletableToken { label: "create_object_group", lower: "create_object_group", kind: CompletionItemKind::FUNCTION, snippet: Some("create_object_group ${1:GROUP_NAME} {\n\t$0\n}") },
    CompletableToken { label: "create_object", lower: "create_object", kind: CompletionItemKind::FUNCTION, snippet: Some("create_object ${1:OBJECT} {\n\t$0\n}") },
    // Attributes
    CompletableToken { label: "terrain_type", lower: "terrain_type", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_type ${1:TERRAIN}") },
    CompletableToken { label: "land_percent", lower: "land_percent", kind: CompletionItemKind::PROPERTY, snippet: Some("land_percent ${1:PERCENT}") },
    CompletableToken { label: "number_of_tiles", lower: "number_of_tiles", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_tiles ${1:NUMBER}") },
    CompletableToken { label: "base_size", lower: "base_size", kind: CompletionItemKind::PROPERTY, snippet: Some("base_size ${1:RADIUS}") },
    CompletableToken { label: "set_circular_base", lower: "set_circular_base", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "generate_mode", lower: "generate_mode", kind: CompletionItemKind::PROPERTY, snippet: Some("generate_mode ${1:MODE}") },
    CompletableToken { label: "land_position", lower: "land_position", kind: CompletionItemKind::PROPERTY, snippet: Some("land_position ${1:X} ${2:Y}") },
    CompletableToken { label: "circle_radius", lower: "circle_radius", kind: CompletionItemKind::PROPERTY, snippet: Some("circle_radius ${1:RADIUS} ${2:VARIANCE}") },
    CompletableToken { label: "left_border", lower: "left_border", kind: CompletionItemKind::PROPERTY, snippet: Some("left_border ${1:PERCENT}") },
    CompletableToken { label: "right_border", lower: "right_border", kind: CompletionItemKind::PROPERTY, snippet: Some("right_border ${1:PERCENT}") },
    CompletableToken { label: "top_border", lower: "top_border", kind: CompletionItemKind::PROPERTY, snippet: Some("top_border ${1:PERCENT}") },
    CompletableToken { label: "bottom_border", lower: "bottom_border", kind: CompletionItemKind::PROPERTY, snippet: Some("bottom_border ${1:PERCENT}") },
    CompletableToken { label: "border_fuzziness", lower: "border_fuzziness", kind: CompletionItemKind::PROPERTY, snippet: Some("border_fuzziness ${1:PERCENT}") },
    CompletableToken { label: "clumping_factor", lower: "clumping_factor", kind: CompletionItemKind::PROPERTY, snippet: Some("clumping_factor ${1:FACTOR}") },
    CompletableToken { label: "land_conformity", lower: "land_conformity", kind: CompletionItemKind::PROPERTY, snippet: Some("land_conformity ${1:PERCENT}") },
    CompletableToken { label: "base_elevation", lower: "base_elevation", kind: CompletionItemKind::PROPERTY, snippet: Some("base_elevation ${1:HEIGHT}") },
    CompletableToken { label: "assign_to_player", lower: "assign_to_player", kind: CompletionItemKind::PROPERTY, snippet: Some("assign_to_player ${1:PLAYER_NUMBER}") },
    CompletableToken { label: "assign_to", lower: "assign_to", kind: CompletionItemKind::PROPERTY, snippet: Some("assign_to ${1:ASSIGN_TARGET} ${2:NUMBER} ${3:MODE} ${4:FLAGS}") },
    CompletableToken { label: "zone", lower: "zone", kind: CompletionItemKind::PROPERTY, snippet: Some("zone ${1:ZONE_NUMBER}") },
    CompletableToken { label: "set_zone_by_team", lower: "set_zone_by_team", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_zone_randomly", lower: "set_zone_randomly", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "other_zone_avoidance_distance", lower: "other_zone_avoidance_distance", kind: CompletionItemKind::PROPERTY, snippet: Some("other_zone_avoidance_distance ${1:TILES}") },
    CompletableToken { label: "min_placement_distance", lower: "min_placement_distance", kind: CompletionItemKind::PROPERTY, snippet: Some("min_placement_distance ${1:TILES}") },
    CompletableToken { label: "land_id", lower: "land_id", kind: CompletionItemKind::PROPERTY, snippet: Some("land_id ${1:LAND_ID_NUMBER}") },
    CompletableToken { label: "number_of_clumps", lower: "number_of_clumps", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_clumps ${1:NUMBER}") },
    CompletableToken { label: "set_scale_by_size", lower: "set_scale_by_size", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_scale_by_groups", lower: "set_scale_by_groups", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "spacing", lower: "spacing", kind: CompletionItemKind::PROPERTY, snippet: Some("spacing ${1:NUMBER}") },
    CompletableToken { label: "enable_balanced_elevation", lower: "enable_balanced_elevation", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "beach_terrain", lower: "beach_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("beach_terrain ${1:TERRAIN}") },
    CompletableToken { label: "terrain_mask", lower: "terrain_mask", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_mask ${1:LAYER}") },
    CompletableToken { label: "spacing_to_other_terrain_types", lower: "spacing_to_other_terrain_types", kind: CompletionItemKind::PROPERTY, snippet: Some("spacing_to_other_terrain_types ${1:DISTANCE}") },
    CompletableToken { label: "spacing_to_specific_terrain", lower: "spacing_to_specific_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("spacing_to_specific_terrain ${1:TERRAIN} ${2:DISTANCE}") },
    CompletableToken { label: "set_flat_terrain_only", lower: "set_flat_terrain_only", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_avoid_player_start_areas", lower: "set_avoid_player_start_areas", kind: CompletionItemKind::PROPERTY, snippet: Some("set_avoid_player_start_areas ${1:DISTANCE}") },
    CompletableToken { label: "height_limits", lower: "height_limits", kind: CompletionItemKind::PROPERTY, snippet: Some("height_limits ${1:MIN} ${2:MAX}") },
    CompletableToken { label: "default_terrain_replacement", lower: "default_terrain_replacement", kind: CompletionItemKind::PROPERTY, snippet: Some("default_terrain_replacement ${1:TERRAIN}") },
    CompletableToken { label: "replace_terrain", lower: "replace_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("replace_terrain ${1:OLD_TERRAIN} ${2:NEW_TERRAIN}") },
    CompletableToken { label: "terrain_cost", lower: "terrain_cost", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_cost ${1:TERRAIN} ${2:COST}") },
    CompletableToken { label: "terrain_size", lower: "terrain_size", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_size ${1:TERRAIN} ${2:RADIUS} ${3:VARIANCE}") },
    CompletableToken { label: "add_object", lower: "add_object", kind: CompletionItemKind::PROPERTY, snippet: Some("add_object ${1:OBJECT} ${2:PERCENT}") },
    CompletableToken { label: "number_of_objects", lower: "number_of_objects", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_objects ${1:NUMBER}") },
    CompletableToken { label: "number_of_groups", lower: "number_of_groups", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_groups ${1:NUMBER}") },
    CompletableToken { label: "group_variance", lower: "group_variance", kind: CompletionItemKind::PROPERTY, snippet: Some("group_variance ${1:VARIANCE}") },
    CompletableToken { label: "group_placement_radius", lower: "group_placement_radius", kind: CompletionItemKind::PROPERTY, snippet: Some("group_placement_radius ${1:RADIUS}") },
    CompletableToken { label: "set_tight_grouping", lower: "set_tight_grouping", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_loose_grouping", lower: "set_loose_grouping", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "min_connected_tiles", lower: "min_connected_tiles", kind: CompletionItemKind::PROPERTY, snippet: Some("min_connected_tiles ${1:NUMBER}") },
    CompletableToken { label: "resource_delta", lower: "resource_delta", kind: CompletionItemKind::PROPERTY, snippet: Some("resource_delta ${1:NUMBER}") },
    CompletableToken { label: "second_object", lower: "second_object", kind: CompletionItemKind::PROPERTY, snippet: Some("second_object ${1:OBJECT}") },
    CompletableToken { label: "set_scaling_to_map_size", lower: "set_scaling_to_map_size", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_scaling_to_player_number", lower: "set_scaling_to_player_number", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_place_for_every_player", lower: "set_place_for_every_player", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "place_on_specific_land_id", lower: "place_on_specific_land_id", kind: CompletionItemKind::PROPERTY, snippet: Some("place_on_specific_land_id ${1:LAND_ID}") },
    CompletableToken { label: "avoid_other_land_zones", lower: "avoid_other_land_zones", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_other_land_zones ${1:DISTANCE}") },
    CompletableToken { label: "generate_for_first_land_only", lower: "generate_for_first_land_only", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_gaia_object_only", lower: "set_gaia_object_only", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_gaia_unconvertible", lower: "set_gaia_unconvertible", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_building_capturable", lower: "set_building_capturable", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "make_indestructible", lower: "make_indestructible", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "min_distance_to_players", lower: "min_distance_to_players", kind: CompletionItemKind::PROPERTY, snippet: Some("min_distance_to_players ${1:DISTANCE}") },
    CompletableToken { label: "max_distance_to_players", lower: "max_distance_to_players", kind: CompletionItemKind::PROPERTY, snippet: Some("max_distance_to_players ${1:DISTANCE}") },
    CompletableToken { label: "set_circular_placement", lower: "set_circular_placement", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "terrain_to_place_on", lower: "terrain_to_place_on", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_to_place_on ${1:TERRAIN}") },
    CompletableToken { label: "layer_to_place_on", lower: "layer_to_place_on", kind: CompletionItemKind::PROPERTY, snippet: Some("layer_to_place_on ${1:TERRAIN}") },
    CompletableToken { label: "ignore_terrain_restrictions", lower: "ignore_terrain_restrictions", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "max_distance_to_other_zones", lower: "max_distance_to_other_zones", kind: CompletionItemKind::PROPERTY, snippet: Some("max_distance_to_other_zones ${1:DISTANCE}") },
    CompletableToken { label: "place_on_forest_zone", lower: "place_on_forest_zone", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "avoid_forest_zone", lower: "avoid_forest_zone", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_forest_zone ${1:DISTANCE}") },
    CompletableToken { label: "avoid_cliff_zone", lower: "avoid_cliff_zone", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_cliff_zone ${1:DISTANCE}") },
    CompletableToken { label: "min_distance_to_map_edge", lower: "min_distance_to_map_edge", kind: CompletionItemKind::PROPERTY, snippet: Some("min_distance_to_map_edge ${1:DISTANCE}") },
    CompletableToken { label: "min_distance_group_placement", lower: "min_distance_group_placement", kind: CompletionItemKind::PROPERTY, snippet: Some("min_distance_group_placement ${1:DISTANCE}") },
    CompletableToken { label: "temp_min_distance_group_placement", lower: "temp_min_distance_group_placement", kind: CompletionItemKind::PROPERTY, snippet: Some("temp_min_distance_group_placement ${1:DISTANCE}") },
    CompletableToken { label: "find_closest", lower: "find_closest", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "find_closest_to_map_center", lower: "find_closest_to_map_center", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "find_closest_to_map_edge", lower: "find_closest_to_map_edge", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "enable_tile_shuffling", lower: "enable_tile_shuffling", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "require_path", lower: "require_path", kind: CompletionItemKind::PROPERTY, snippet: Some("require_path ${1:DEVIATION}") },
    CompletableToken { label: "force_placement", lower: "force_placement", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "actor_area", lower: "actor_area", kind: CompletionItemKind::PROPERTY, snippet: Some("actor_area ${1:ACTOR_AREA_ID}") },
    CompletableToken { label: "actor_area_radius", lower: "actor_area_radius", kind: CompletionItemKind::PROPERTY, snippet: Some("actor_area_radius ${1:RADIUS}") },
    CompletableToken { label: "override_actor_radius_if_required", lower: "override_actor_radius_if_required", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "actor_area_to_place_in", lower: "actor_area_to_place_in", kind: CompletionItemKind::PROPERTY, snippet: Some("actor_area_to_place_in ${1:ACTOR_AREA_ID}") },
    CompletableToken { label: "avoid_actor_area", lower: "avoid_actor_area", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_actor_area ${1:ACTOR_AREA_ID}") },
    CompletableToken { label: "avoid_all_actor_areas", lower: "avoid_all_actor_areas", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_facet", lower: "set_facet", kind: CompletionItemKind::PROPERTY, snippet: Some("set_facet ${1:FACET_NUMBER}") },
    CompletableToken { label: "match_player_civ", lower: "match_player_civ", kind: CompletionItemKind::PROPERTY, snippet: None },
    // Command-Attributes
    CompletableToken { label: "base_terrain", lower: "base_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("base_terrain ${1:TERRAIN}") },
    CompletableToken { label: "base_layer", lower: "base_layer", kind: CompletionItemKind::PROPERTY, snippet: Some("base_layer ${1:TERRAIN}") },
    // Keywords
    CompletableToken { label: "else", lower: "else", kind: CompletionItemKind::KEYWORD, snippet: None },
    CompletableToken { label: "if", lower: "if", kind: CompletionItemKind::KEYWORD, snippet: Some("if ${1:CONDITION}\n\t$0\nendif") },
    CompletableToken { label: "elseif", lower: "elseif", kind: CompletionItemKind::KEYWORD, snippet: Some("elseif ${1:CONDITION}") },
    CompletableToken { label: "endif", lower: "endif", kind: CompletionItemKind::KEYWORD, snippet: None },
    CompletableToken { label: "start_random", lower: "start_random", kind: CompletionItemKind::KEYWORD, snippet: Some("start_random\n\tpercent_chance ${1:PERCENT} $0\nend_random") },
    CompletableToken { label: "end_random", lower: "end_random", kind: CompletionItemKind::KEYWORD, snippet: None },
    CompletableToken { label: "percent_chance", lower: "percent_chance", kind: CompletionItemKind::KEYWORD, snippet: Some("percent_chance ${1:PERCENT} $0") },
    CompletableToken { label: "rnd", lower: "rnd", kind: CompletionItemKind::KEYWORD, snippet: Some("rnd(${1:min},${2:max})") },
    CompletableToken { label: "#define", lower: "#define", kind: CompletionItemKind::KEYWORD, snippet: Some("#define ${1:LABEL}") },
    CompletableToken { label: "#const", lower: "#const", kind: CompletionItemKind::KEYWORD, snippet: Some("#const ${1:IDENTIFIER} ${2:VALUE}") },
    CompletableToken { label: "#include_drs", lower: "#include_drs", kind: CompletionItemKind::KEYWORD, snippet: Some("#include_drs ${1:FILEPATH}") },
    CompletableToken { label: "#includeXS", lower: "#includexs", kind: CompletionItemKind::KEYWORD, snippet: Some("#includeXS ${1:FILEPATH}") },
];

/// A token that can be offered as a completion suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CompletableToken {
    /// The display label shown in the completion popup, with correct casing.
    label: &'static str,
    /// The lowercase version of the label, used for case-insensitive matching.
    lower: &'static str,
    /// The kind of token, used for coloring and icons in the completion popup.
    kind: CompletionItemKind,
    /// The snippet to insert when the completion is accepted, or `None` if the
    /// label should be inserted as-is. Uses LSP snippet syntax where `${N:placeholder}`
    /// marks tab stops and `$0` marks the final cursor position.
    snippet: Option<&'static str>,
}

impl CompletableToken {
    /// Creates a `CompletionItem` for this token.
    /// When the completion is accepted, `range` is replaced with the token's
    /// snippet if one exists, or the label otherwise.
    fn to_completion_item(&self, range: Range) -> CompletionItem {
        CompletionItem {
            label: self.label.to_string(),
            kind: Some(self.kind),
            insert_text_format: self.snippet.map(|_| InsertTextFormat::SNIPPET),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range,
                new_text: self.snippet.unwrap_or(self.label).to_string(),
            })),
            documentation: lookup_hover(self.label).map(|doc| {
                Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: doc.to_string(),
                })
            }),
            ..Default::default()
        }
    }
}

/// Returns the completable tokens matching the given completion text.
///
/// First attempts to match against the full token. If no matches are found,
/// falls back to matching against the prefix up to the cursor position.
/// This ensures that partially typed tokens that don't yet match anything
/// still surface relevant completions based on what has been typed so far.
///
/// For example, if the user types `create_xyz` with the cursor after `create_`,
/// the full token `"create_xyz"` matches nothing, but the prefix `"create_"`
/// matches all `create_` commands.
fn filter_tokens(completion_text: &CompletionText) -> Vec<&'static CompletableToken> {
    let matches: Vec<&CompletableToken> = COMPLETABLE_TOKENS
        .iter()
        .filter(|token| token.lower.contains(&completion_text.text))
        .collect();
    if matches.is_empty() {
        COMPLETABLE_TOKENS
            .iter()
            .filter(|token| token.lower.contains(&completion_text.prefix))
            .collect()
    } else {
        matches
    }
}

/// Returns a completion response for the given text and position.
///
/// The `COMPLETABLE_TOKENS` list defines the tokens that can be completed.
/// A valid completion occurs when the prefix at the current position is
/// contained in a token from the `COMPLETABLE_TOKENS` list.
/// Comparisons are case-insensitive.
///
/// Returns `None` if the position is in a comment.
/// Returns the entire list if there is no prefix to complete.
pub fn get_completions(text: &str, position: Position) -> Option<CompletionResponse> {
    // No autocomplete in comments.
    let context = rms::document_context_at(text, position);
    if context.in_comment {
        return None;
    }

    // Note the empty string matches everything.
    let completion_text = rms::extract_autocomplete_prefix(text, position)?;
    let matches: Vec<&CompletableToken> = filter_tokens(&completion_text);
    let completion_items = matches
        .iter()
        .map(|token| token.to_completion_item(completion_text.range))
        .collect();
    Some(CompletionResponse::Array(completion_items))
}
