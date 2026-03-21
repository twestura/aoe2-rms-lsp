//! Aoe2 RMS Language Server

mod rms;

use std::collections::HashMap;

use tokio::sync::RwLock;
use tower_lsp::Client;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
    CompletionTextEdit, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Documentation, Hover, HoverContents, HoverParams,
    HoverProviderCapability, InsertTextFormat, MarkupContent, MarkupKind, Position,
    PositionEncodingKind, Range, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextEdit, Url,
};
use tower_lsp::{
    LanguageServer, LspService, Server,
    lsp_types::{InitializeParams, InitializeResult, InitializedParams, MessageType},
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
    CompletableToken { label: "behavior_version", lower: "behavior_version", kind: CompletionItemKind::FUNCTION, snippet: Some("behavior_version ${1:N}") },
    CompletableToken { label: "override_map_size", lower: "override_map_size", kind: CompletionItemKind::FUNCTION, snippet: Some("override_map_size ${1:N}") },
    CompletableToken { label: "set_gaia_civilization", lower: "set_gaia_civilization", kind: CompletionItemKind::FUNCTION, snippet: Some("set_gaia_civilization ${1:N}") },
    CompletableToken { label: "ai_info_map_type", lower: "ai_info_map_type", kind: CompletionItemKind::FUNCTION, snippet: Some("ai_info_map_type ${1:TYPE} ${2:0} ${3:0} ${4:0}") },
    CompletableToken { label: "effect_amount", lower: "effect_amount", kind: CompletionItemKind::FUNCTION, snippet: Some("effect_amount ${1:EFFECT_TYPE} ${2:TYPE} ${3:ATTR} ${4:N}") },
    CompletableToken { label: "effect_percent", lower: "effect_percent", kind: CompletionItemKind::FUNCTION, snippet: Some("effect_percent ${1:EFFECT_TYPE} ${2:TYPE} ${3:ATTR} ${4:N}") },
    CompletableToken { label: "guard_state", lower: "guard_state", kind: CompletionItemKind::FUNCTION, snippet: Some("guard_state ${1:TYPE} ${2:RESOURCE} ${3:0} ${4:0}") },
    CompletableToken { label: "terrain_state", lower: "terrain_state", kind: CompletionItemKind::FUNCTION, snippet: Some("terrain_state ${1:0} ${2:0} ${3:0} ${4:N}") },
    CompletableToken { label: "weather_type", lower: "weather_type", kind: CompletionItemKind::FUNCTION, snippet: Some("weather_type ${1:N} ${2:N} ${3:N} ${4:N}") },
    CompletableToken { label: "water_definition", lower: "water_definition", kind: CompletionItemKind::FUNCTION, snippet: Some("water_definition ${1:TYPE}") },
    CompletableToken { label: "enable_waves", lower: "enable_waves", kind: CompletionItemKind::FUNCTION, snippet: Some("enable_waves ${1:N}") },
    CompletableToken { label: "create_player_lands", lower: "create_player_lands", kind: CompletionItemKind::FUNCTION, snippet: Some("create_player_lands {\n\t$0\n}") },
    CompletableToken { label: "create_land", lower: "create_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_land {\n\t$0\n}") },
    CompletableToken { label: "create_elevation", lower: "create_elevation", kind: CompletionItemKind::FUNCTION, snippet: Some("create_elevation ${1:N} {\n\t$0\n}") },
    CompletableToken { label: "color_correction", lower: "color_correction", kind: CompletionItemKind::FUNCTION, snippet: Some("color_correction ${1:TYPE}") },
    CompletableToken { label: "create_terrain", lower: "create_terrain", kind: CompletionItemKind::FUNCTION, snippet: Some("create_terrain ${1:TYPE} {\n\t$0\n}") },
    CompletableToken { label: "cliff_type", lower: "cliff_type", kind: CompletionItemKind::FUNCTION, snippet: Some("cliff_type ${1:TYPE}") },
    CompletableToken { label: "min_number_of_cliffs", lower: "min_number_of_cliffs", kind: CompletionItemKind::FUNCTION, snippet: Some("min_number_of_cliffs ${1:N}") },
    CompletableToken { label: "max_number_of_cliffs", lower: "max_number_of_cliffs", kind: CompletionItemKind::FUNCTION, snippet: Some("max_number_of_cliffs ${1:N}") },
    CompletableToken { label: "min_length_of_cliff", lower: "min_length_of_cliff", kind: CompletionItemKind::FUNCTION, snippet: Some("min_length_of_cliff ${1:N}") },
    CompletableToken { label: "max_length_of_cliff", lower: "max_length_of_cliff", kind: CompletionItemKind::FUNCTION, snippet: Some("max_length_of_cliff ${1:N}") },
    CompletableToken { label: "cliff_curliness", lower: "cliff_curliness", kind: CompletionItemKind::FUNCTION, snippet: Some("cliff_curliness ${1:N}") },
    CompletableToken { label: "min_distance_cliffs", lower: "min_distance_cliffs", kind: CompletionItemKind::FUNCTION, snippet: Some("min_distance_cliffs ${1:N}") },
    CompletableToken { label: "min_terrain_distance", lower: "min_terrain_distance", kind: CompletionItemKind::FUNCTION, snippet: Some("min_terrain_distance ${1:N}") },
    CompletableToken { label: "accumulate_connections", lower: "accumulate_connections", kind: CompletionItemKind::FUNCTION, snippet: None },
    CompletableToken { label: "create_connect_all_players_land", lower: "create_connect_all_players_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_all_players_land {\n\t$0\n}") },
    CompletableToken { label: "create_connect_teams_lands", lower: "create_connect_teams_lands", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_teams_lands {\n\t$0\n}") },
    CompletableToken { label: "create_connect_all_lands", lower: "create_connect_all_lands", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_all_lands {\n\t$0\n}") },
    CompletableToken { label: "create_connect_same_land_zones", lower: "create_connect_same_land_zones", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_same_land_zones {\n\t$0\n}") },
    CompletableToken { label: "create_connect_land_zones", lower: "create_connect_land_zones", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_land_zones ${1:N} ${2:N} {\n\t$0\n}") },
    CompletableToken { label: "create_connect_to_nonplayer_land", lower: "create_connect_to_nonplayer_land", kind: CompletionItemKind::FUNCTION, snippet: Some("create_connect_to_nonplayer_land {\n\t$0\n}") },
    CompletableToken { label: "create_actor_area", lower: "create_actor_area", kind: CompletionItemKind::FUNCTION, snippet: Some("create_actor_area ${1:X} ${2:Y} ${3:ID} ${4:RADIUS}") },
    CompletableToken { label: "create_object_group", lower: "create_object_group", kind: CompletionItemKind::FUNCTION, snippet: Some("create_object_group ${1:NAME} {\n\t$0\n}") },
    CompletableToken { label: "create_object", lower: "create_object", kind: CompletionItemKind::FUNCTION, snippet: Some("create_object ${1:TYPE} {\n\t$0\n}") },
    // Attributes
    CompletableToken { label: "terrain_type", lower: "terrain_type", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_type ${1:TYPE}") },
    CompletableToken { label: "land_percent", lower: "land_percent", kind: CompletionItemKind::PROPERTY, snippet: Some("land_percent ${1:N}") },
    CompletableToken { label: "number_of_tiles", lower: "number_of_tiles", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_tiles ${1:N}") },
    CompletableToken { label: "base_size", lower: "base_size", kind: CompletionItemKind::PROPERTY, snippet: Some("base_size ${1:N}") },
    CompletableToken { label: "set_circular_base", lower: "set_circular_base", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "generate_mode", lower: "generate_mode", kind: CompletionItemKind::PROPERTY, snippet: Some("generate_mode ${1:N}") },
    CompletableToken { label: "land_position", lower: "land_position", kind: CompletionItemKind::PROPERTY, snippet: Some("land_position ${1:X} ${2:Y}") },
    CompletableToken { label: "circle_radius", lower: "circle_radius", kind: CompletionItemKind::PROPERTY, snippet: Some("circle_radius ${1:N} ${2:N}") },
    CompletableToken { label: "left_border", lower: "left_border", kind: CompletionItemKind::PROPERTY, snippet: Some("left_border ${1:N}") },
    CompletableToken { label: "right_border", lower: "right_border", kind: CompletionItemKind::PROPERTY, snippet: Some("right_border ${1:N}") },
    CompletableToken { label: "top_border", lower: "top_border", kind: CompletionItemKind::PROPERTY, snippet: Some("top_border ${1:N}") },
    CompletableToken { label: "bottom_border", lower: "bottom_border", kind: CompletionItemKind::PROPERTY, snippet: Some("bottom_border ${1:N}") },
    CompletableToken { label: "border_fuzziness", lower: "border_fuzziness", kind: CompletionItemKind::PROPERTY, snippet: Some("border_fuzziness ${1:N}") },
    CompletableToken { label: "clumping_factor", lower: "clumping_factor", kind: CompletionItemKind::PROPERTY, snippet: Some("clumping_factor ${1:N}") },
    CompletableToken { label: "land_conformity", lower: "land_conformity", kind: CompletionItemKind::PROPERTY, snippet: Some("land_conformity ${1:N}") },
    CompletableToken { label: "base_elevation", lower: "base_elevation", kind: CompletionItemKind::PROPERTY, snippet: Some("base_elevation ${1:N}") },
    CompletableToken { label: "assign_to_player", lower: "assign_to_player", kind: CompletionItemKind::PROPERTY, snippet: Some("assign_to_player ${1:N}") },
    CompletableToken { label: "assign_to", lower: "assign_to", kind: CompletionItemKind::PROPERTY, snippet: Some("assign_to ${1:TARGET} ${2:N} ${3:0} ${4:0}") },
    CompletableToken { label: "zone", lower: "zone", kind: CompletionItemKind::PROPERTY, snippet: Some("zone ${1:N}") },
    CompletableToken { label: "set_zone_by_team", lower: "set_zone_by_team", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_zone_randomly", lower: "set_zone_randomly", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "other_zone_avoidance_distance", lower: "other_zone_avoidance_distance", kind: CompletionItemKind::PROPERTY, snippet: Some("other_zone_avoidance_distance ${1:N}") },
    CompletableToken { label: "min_placement_distance", lower: "min_placement_distance", kind: CompletionItemKind::PROPERTY, snippet: Some("min_placement_distance ${1:N}") },
    CompletableToken { label: "land_id", lower: "land_id", kind: CompletionItemKind::PROPERTY, snippet: Some("land_id ${1:N}") },
    CompletableToken { label: "number_of_clumps", lower: "number_of_clumps", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_clumps ${1:N}") },
    CompletableToken { label: "set_scale_by_size", lower: "set_scale_by_size", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_scale_by_groups", lower: "set_scale_by_groups", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "spacing", lower: "spacing", kind: CompletionItemKind::PROPERTY, snippet: Some("spacing ${1:N}") },
    CompletableToken { label: "enable_balanced_elevation", lower: "enable_balanced_elevation", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "beach_terrain", lower: "beach_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("beach_terrain ${1:TYPE}") },
    CompletableToken { label: "terrain_mask", lower: "terrain_mask", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_mask ${1:N}") },
    CompletableToken { label: "spacing_to_other_terrain_types", lower: "spacing_to_other_terrain_types", kind: CompletionItemKind::PROPERTY, snippet: Some("spacing_to_other_terrain_types ${1:N}") },
    CompletableToken { label: "spacing_to_specific_terrain", lower: "spacing_to_specific_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("spacing_to_specific_terrain ${1:TYPE} ${2:N}") },
    CompletableToken { label: "set_flat_terrain_only", lower: "set_flat_terrain_only", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_avoid_player_start_areas", lower: "set_avoid_player_start_areas", kind: CompletionItemKind::PROPERTY, snippet: Some("set_avoid_player_start_areas ${1:N}") },
    CompletableToken { label: "height_limits", lower: "height_limits", kind: CompletionItemKind::PROPERTY, snippet: Some("height_limits ${1:N} ${2:N}") },
    CompletableToken { label: "default_terrain_replacement", lower: "default_terrain_replacement", kind: CompletionItemKind::PROPERTY, snippet: Some("default_terrain_replacement ${1:TYPE}") },
    CompletableToken { label: "replace_terrain", lower: "replace_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("replace_terrain ${1:TYPE} ${2:TYPE}") },
    CompletableToken { label: "terrain_cost", lower: "terrain_cost", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_cost ${1:TYPE} ${2:N}") },
    CompletableToken { label: "terrain_size", lower: "terrain_size", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_size ${1:TYPE} ${2:N} ${3:N}") },
    CompletableToken { label: "add_object", lower: "add_object", kind: CompletionItemKind::PROPERTY, snippet: Some("add_object ${1:TYPE} ${2:N}") },
    CompletableToken { label: "number_of_objects", lower: "number_of_objects", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_objects ${1:N}") },
    CompletableToken { label: "number_of_groups", lower: "number_of_groups", kind: CompletionItemKind::PROPERTY, snippet: Some("number_of_groups ${1:N}") },
    CompletableToken { label: "group_variance", lower: "group_variance", kind: CompletionItemKind::PROPERTY, snippet: Some("group_variance ${1:N}") },
    CompletableToken { label: "group_placement_radius", lower: "group_placement_radius", kind: CompletionItemKind::PROPERTY, snippet: Some("group_placement_radius ${1:N}") },
    CompletableToken { label: "set_tight_grouping", lower: "set_tight_grouping", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_loose_grouping", lower: "set_loose_grouping", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "min_connected_tiles", lower: "min_connected_tiles", kind: CompletionItemKind::PROPERTY, snippet: Some("min_connected_tiles ${1:N}") },
    CompletableToken { label: "resource_delta", lower: "resource_delta", kind: CompletionItemKind::PROPERTY, snippet: Some("resource_delta ${1:N}") },
    CompletableToken { label: "second_object", lower: "second_object", kind: CompletionItemKind::PROPERTY, snippet: Some("second_object ${1:TYPE}") },
    CompletableToken { label: "set_scaling_to_map_size", lower: "set_scaling_to_map_size", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_scaling_to_player_number", lower: "set_scaling_to_player_number", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_place_for_every_player", lower: "set_place_for_every_player", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "place_on_specific_land_id", lower: "place_on_specific_land_id", kind: CompletionItemKind::PROPERTY, snippet: Some("place_on_specific_land_id ${1:N}") },
    CompletableToken { label: "avoid_other_land_zones", lower: "avoid_other_land_zones", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_other_land_zones ${1:N}") },
    CompletableToken { label: "generate_for_first_land_only", lower: "generate_for_first_land_only", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_gaia_object_only", lower: "set_gaia_object_only", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_gaia_unconvertible", lower: "set_gaia_unconvertible", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_building_capturable", lower: "set_building_capturable", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "make_indestructible", lower: "make_indestructible", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "min_distance_to_players", lower: "min_distance_to_players", kind: CompletionItemKind::PROPERTY, snippet: Some("min_distance_to_players ${1:N}") },
    CompletableToken { label: "max_distance_to_players", lower: "max_distance_to_players", kind: CompletionItemKind::PROPERTY, snippet: Some("max_distance_to_players ${1:N}") },
    CompletableToken { label: "set_circular_placement", lower: "set_circular_placement", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "terrain_to_place_on", lower: "terrain_to_place_on", kind: CompletionItemKind::PROPERTY, snippet: Some("terrain_to_place_on ${1:TYPE}") },
    CompletableToken { label: "layer_to_place_on", lower: "layer_to_place_on", kind: CompletionItemKind::PROPERTY, snippet: Some("layer_to_place_on ${1:TYPE}") },
    CompletableToken { label: "ignore_terrain_restrictions", lower: "ignore_terrain_restrictions", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "max_distance_to_other_zones", lower: "max_distance_to_other_zones", kind: CompletionItemKind::PROPERTY, snippet: Some("max_distance_to_other_zones ${1:N}") },
    CompletableToken { label: "place_on_forest_zone", lower: "place_on_forest_zone", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "avoid_forest_zone", lower: "avoid_forest_zone", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_forest_zone ${1:N}") },
    CompletableToken { label: "avoid_cliff_zone", lower: "avoid_cliff_zone", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_cliff_zone ${1:N}") },
    CompletableToken { label: "min_distance_to_map_edge", lower: "min_distance_to_map_edge", kind: CompletionItemKind::PROPERTY, snippet: Some("min_distance_to_map_edge ${1:N}") },
    CompletableToken { label: "min_distance_group_placement", lower: "min_distance_group_placement", kind: CompletionItemKind::PROPERTY, snippet: Some("min_distance_group_placement ${1:N}") },
    CompletableToken { label: "temp_min_distance_group_placement", lower: "temp_min_distance_group_placement", kind: CompletionItemKind::PROPERTY, snippet: Some("temp_min_distance_group_placement ${1:N}") },
    CompletableToken { label: "find_closest", lower: "find_closest", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "find_closest_to_map_center", lower: "find_closest_to_map_center", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "find_closest_to_map_edge", lower: "find_closest_to_map_edge", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "enable_tile_shuffling", lower: "enable_tile_shuffling", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "require_path", lower: "require_path", kind: CompletionItemKind::PROPERTY, snippet: Some("require_path ${1:N}") },
    CompletableToken { label: "force_placement", lower: "force_placement", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "actor_area", lower: "actor_area", kind: CompletionItemKind::PROPERTY, snippet: Some("actor_area ${1:N}") },
    CompletableToken { label: "actor_area_radius", lower: "actor_area_radius", kind: CompletionItemKind::PROPERTY, snippet: Some("actor_area_radius ${1:N}") },
    CompletableToken { label: "override_actor_radius_if_required", lower: "override_actor_radius_if_required", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "actor_area_to_place_in", lower: "actor_area_to_place_in", kind: CompletionItemKind::PROPERTY, snippet: Some("actor_area_to_place_in ${1:N}") },
    CompletableToken { label: "avoid_actor_area", lower: "avoid_actor_area", kind: CompletionItemKind::PROPERTY, snippet: Some("avoid_actor_area ${1:N}") },
    CompletableToken { label: "avoid_all_actor_areas", lower: "avoid_all_actor_areas", kind: CompletionItemKind::PROPERTY, snippet: None },
    CompletableToken { label: "set_facet", lower: "set_facet", kind: CompletionItemKind::PROPERTY, snippet: Some("set_facet ${1:N}") },
    CompletableToken { label: "match_player_civ", lower: "match_player_civ", kind: CompletionItemKind::PROPERTY, snippet: None },
    // Command-Attributes
    CompletableToken { label: "base_terrain", lower: "base_terrain", kind: CompletionItemKind::PROPERTY, snippet: Some("base_terrain ${1:TYPE}") },
    CompletableToken { label: "base_layer", lower: "base_layer", kind: CompletionItemKind::PROPERTY, snippet: Some("base_layer ${1:TYPE}") },
    // Keywords
    CompletableToken { label: "else", lower: "else", kind: CompletionItemKind::KEYWORD, snippet: None },
    CompletableToken { label: "if", lower: "if", kind: CompletionItemKind::KEYWORD, snippet: Some("if ${1:CONDITION}\n\t$0\nendif") },
    CompletableToken { label: "elseif", lower: "elseif", kind: CompletionItemKind::KEYWORD, snippet: Some("elseif ${1:CONDITION}") },
    CompletableToken { label: "endif", lower: "endif", kind: CompletionItemKind::KEYWORD, snippet: None },
    CompletableToken { label: "start_random", lower: "start_random", kind: CompletionItemKind::KEYWORD, snippet: Some("start_random\n\tpercent_chance ${1:N} $0\nend_random") },
    CompletableToken { label: "end_random", lower: "end_random", kind: CompletionItemKind::KEYWORD, snippet: None },
    CompletableToken { label: "percent_chance", lower: "percent_chance", kind: CompletionItemKind::KEYWORD, snippet: Some("percent_chance ${1:N} $0") },
    CompletableToken { label: "rnd", lower: "rnd", kind: CompletionItemKind::KEYWORD, snippet: Some("rnd(${1:min},${2:max})") },
    CompletableToken { label: "#define", lower: "#define", kind: CompletionItemKind::KEYWORD, snippet: Some("#define ${1:CONDITION}") },
    CompletableToken { label: "#const", lower: "#const", kind: CompletionItemKind::KEYWORD, snippet: Some("#const ${1:NAME} ${2:N}") },
    CompletableToken { label: "#include_drs", lower: "#include_drs", kind: CompletionItemKind::KEYWORD, snippet: Some("#include_drs ${1:FILENAME}") },
    CompletableToken { label: "#includeXS", lower: "#includexs", kind: CompletionItemKind::KEYWORD, snippet: Some("#includeXS ${1:FILENAME}") },
];

/// The server's in-memory state.
#[derive(Debug)]
struct Backend {
    client: Client,
    /// The server's in-memory document store.
    /// Maps a document's URI to its contents.
    documents: RwLock<HashMap<Url, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                position_encoding: Some(PositionEncodingKind::UTF8),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
        // _log("server initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.documents.write().await.insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        // For now we use the full text document sync kind that returns
        // the entire document in the change. This is fine for small map
        // scripts (many maps are just a few hundred lines) during development.
        let text = params.content_changes.into_iter().next().unwrap().text;
        self.documents.write().await.insert(uri, text);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.read().await;
        let hover_result = documents
            .get(&uri)
            .and_then(|text| get_hover_text(text, position))
            .map(str::to_string)
            .map(|value| Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                range: None,
            });
        Ok(hover_result)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let documents = self.documents.read().await;
        let Some(text) = documents.get(&uri) else {
            return Ok(None);
        };
        let Some(items) = get_completions(text, position) else {
            return Ok(None);
        };
        Ok(Some(CompletionResponse::Array(items)))
    }
}

#[tokio::main]
async fn main() {
    // std::panic::set_hook(Box::new(|info| {
    //     _log(&format!("PANIC: {info}"));
    // }));
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

/// Debugging function for logging messages from the LSP to a file.
fn _log(msg: &str) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("aoe2-rms-lsp.txt")
        .unwrap();
    writeln!(file, "{msg}").unwrap();
}

/// Returns the hover text for the token at the given position, or `None` if there is no text.
fn get_hover_text(text: &str, position: Position) -> Option<&'static str> {
    let context = rms::document_context_at(text, position);
    if context.in_comment {
        return None;
    }
    let token = rms::extract_token(text, position)?;
    lookup_hover(token)
}

/// Returns the hover content for the given token, or `None` if the token does
/// not have hover data.
fn lookup_hover(token: &str) -> Option<&'static str> {
    match token {
        // Sections
        "<PLAYER_SETUP>" => Some(include_str!("../hover_docs/sections/player-setup.md")),
        "<LAND_GENERATION>" => Some(include_str!("../hover_docs/sections/land-generation.md")),
        "<ELEVATION_GENERATION>" => Some(include_str!(
            "../hover_docs/sections/elevation-generation.md"
        )),
        "<CLIFF_GENERATION>" => Some(include_str!("../hover_docs/sections/cliff-generation.md")),
        "<TERRAIN_GENERATION>" => {
            Some(include_str!("../hover_docs/sections/terrain-generation.md"))
        }
        "<CONNECTION_GENERATION>" => Some(include_str!(
            "../hover_docs/sections/connection-generation.md"
        )),
        "<OBJECTS_GENERATION>" => {
            Some(include_str!("../hover_docs/sections/objects-generation.md"))
        }
        // Commands
        "random_placement" => Some(include_str!("../hover_docs/commands/random-placement.md")),
        "direct_placement" => Some(include_str!("../hover_docs/commands/direct-placement.md")),
        "grouped_by_team" => Some(include_str!("../hover_docs/commands/grouped-by-team.md")),
        "nomad_resources" => Some(include_str!("../hover_docs/commands/nomad-resources.md")),
        "force_nomad_treaty" => Some(include_str!("../hover_docs/commands/force-nomad-treaty.md")),
        "behavior_version" => Some(include_str!("../hover_docs/commands/behavior-version.md")),
        "override_map_size" => Some(include_str!("../hover_docs/commands/override-map-size.md")),
        "set_gaia_civilization" => Some(include_str!(
            "../hover_docs/commands/set-gaia-civilization.md"
        )),
        "ai_info_map_type" => Some(include_str!("../hover_docs/commands/ai-info-map-type.md")),
        "effect_amount" => Some(include_str!("../hover_docs/commands/effect-amount.md")),
        "effect_percent" => Some(include_str!("../hover_docs/commands/effect-percent.md")),
        "guard_state" => Some(include_str!("../hover_docs/commands/guard-state.md")),
        "terrain_state" => Some(include_str!("../hover_docs/commands/terrain-state.md")),
        "weather_type" => Some(include_str!("../hover_docs/commands/weather-type.md")),
        "water_definition" => Some(include_str!("../hover_docs/commands/water-definition.md")),
        "enable_waves" => Some(include_str!("../hover_docs/commands/enable-waves.md")),
        "create_player_lands" => Some(include_str!(
            "../hover_docs/commands/create-player-lands.md"
        )),
        "create_land" => Some(include_str!("../hover_docs/commands/create-land.md")),
        "create_elevation" => Some(include_str!("../hover_docs/commands/create-elevation.md")),
        "color_correction" => Some(include_str!("../hover_docs/commands/color-correction.md")),
        "create_terrain" => Some(include_str!("../hover_docs/commands/create-terrain.md")),
        "cliff_type" => Some(include_str!("../hover_docs/commands/cliff-type.md")),
        "min_number_of_cliffs" => Some(include_str!(
            "../hover_docs/commands/min-number-of-cliffs.md"
        )),
        "max_number_of_cliffs" => Some(include_str!(
            "../hover_docs/commands/max-number-of-cliffs.md"
        )),
        "min_length_of_cliff" => Some(include_str!(
            "../hover_docs/commands/min-length-of-cliff.md"
        )),
        "max_length_of_cliff" => Some(include_str!(
            "../hover_docs/commands/max-length-of-cliff.md"
        )),
        "cliff_curliness" => Some(include_str!("../hover_docs/commands/cliff-curliness.md")),
        "min_distance_cliffs" => Some(include_str!(
            "../hover_docs/commands/min-distance-cliffs.md"
        )),
        "min_terrain_distance" => Some(include_str!(
            "../hover_docs/commands/min-terrain-distance.md"
        )),
        "accumulate_connections" => Some(include_str!(
            "../hover_docs/commands/accumulate-connections.md"
        )),
        "create_connect_all_players_land" => Some(include_str!(
            "../hover_docs/commands/create-connect-all-players-land.md"
        )),
        "create_connect_teams_lands" => Some(include_str!(
            "../hover_docs/commands/create-connect-teams-lands.md"
        )),
        "create_connect_all_lands" => Some(include_str!(
            "../hover_docs/commands/create-connect-all-lands.md"
        )),
        "create_connect_same_land_zones" => Some(include_str!(
            "../hover_docs/commands/create-connect-same-land-zones.md"
        )),
        "create_connect_land_zones" => Some(include_str!(
            "../hover_docs/commands/create-connect-land-zones.md"
        )),
        "create_connect_to_nonplayer_land" => Some(include_str!(
            "../hover_docs/commands/create-connect-to-nonplayer-land.md"
        )),
        "create_actor_area" => Some(include_str!("../hover_docs/commands/create-actor-area.md")),
        "create_object_group" => Some(include_str!(
            "../hover_docs/commands/create-object-group.md"
        )),
        "create_object" => Some(include_str!("../hover_docs/commands/create-object.md")),
        // Attributes
        "terrain_type" => Some(include_str!("../hover_docs/attributes/terrain-type.md")),
        "land_percent" => Some(include_str!("../hover_docs/attributes/land-percent.md")),
        "number_of_tiles" => Some(include_str!("../hover_docs/attributes/number-of-tiles.md")),
        "base_size" => Some(include_str!("../hover_docs/attributes/base-size.md")),
        "set_circular_base" => Some(include_str!(
            "../hover_docs/attributes/set-circular-base.md"
        )),
        "generate_mode" => Some(include_str!("../hover_docs/attributes/generate-mode.md")),
        "land_position" => Some(include_str!("../hover_docs/attributes/land-position.md")),
        "circle_radius" => Some(include_str!("../hover_docs/attributes/circle-radius.md")),
        "left_border" => Some(include_str!("../hover_docs/attributes/borders.md")),
        "right_border" => Some(include_str!("../hover_docs/attributes/borders.md")),
        "top_border" => Some(include_str!("../hover_docs/attributes/borders.md")),
        "bottom_border" => Some(include_str!("../hover_docs/attributes/borders.md")),
        "border_fuzziness" => Some(include_str!("../hover_docs/attributes/border-fuzziness.md")),
        "clumping_factor" => Some(include_str!("../hover_docs/attributes/clumping-factor.md")),
        "land_conformity" => Some(include_str!("../hover_docs/attributes/land-conformity.md")),
        "base_elevation" => Some(include_str!("../hover_docs/attributes/base-elevation.md")),
        "assign_to_player" => Some(include_str!("../hover_docs/attributes/assign-to-player.md")),
        "assign_to" => Some(include_str!("../hover_docs/attributes/assign-to.md")),
        "zone" => Some(include_str!("../hover_docs/attributes/zone.md")),
        "set_zone_by_team" => Some(include_str!("../hover_docs/attributes/set-zone-by-team.md")),
        "set_zone_randomly" => Some(include_str!(
            "../hover_docs/attributes/set-zone-randomly.md"
        )),
        "other_zone_avoidance_distance" => Some(include_str!(
            "../hover_docs/attributes/other-zone-avoidance-distance.md"
        )),
        "min_placement_distance" => Some(include_str!(
            "../hover_docs/attributes/min-placement-distance.md"
        )),
        "land_id" => Some(include_str!("../hover_docs/attributes/land-id.md")),
        "number_of_clumps" => Some(include_str!("../hover_docs/attributes/number-of-clumps.md")),
        "set_scale_by_size" => Some(include_str!(
            "../hover_docs/attributes/set-scale-by-size.md"
        )),
        "set_scale_by_groups" => Some(include_str!(
            "../hover_docs/attributes/set-scale-by-groups.md"
        )),
        "spacing" => Some(include_str!("../hover_docs/attributes/spacing.md")),
        "enable_balanced_elevation" => Some(include_str!(
            "../hover_docs/attributes/enable-balanced-elevation.md"
        )),
        "beach_terrain" => Some(include_str!("../hover_docs/attributes/beach-terrain.md")),
        "terrain_mask" => Some(include_str!("../hover_docs/attributes/terrain-mask.md")),
        "spacing_to_other_terrain_types" => Some(include_str!(
            "../hover_docs/attributes/spacing-to-other-terrain-types.md"
        )),
        "spacing_to_specific_terrain" => Some(include_str!(
            "../hover_docs/attributes/spacing-to-specific-terrain.md"
        )),
        "set_flat_terrain_only" => Some(include_str!(
            "../hover_docs/attributes/set-flat-terrain-only.md"
        )),
        "set_avoid_player_start_areas" => Some(include_str!(
            "../hover_docs/attributes/set-avoid-player-start-areas.md"
        )),
        "height_limits" => Some(include_str!("../hover_docs/attributes/height-limits.md")),
        "default_terrain_replacement" => Some(include_str!(
            "../hover_docs/attributes/default-terrain-replacement.md"
        )),
        "replace_terrain" => Some(include_str!("../hover_docs/attributes/replace-terrain.md")),
        "terrain_cost" => Some(include_str!("../hover_docs/attributes/terrain-cost.md")),
        "terrain_size" => Some(include_str!("../hover_docs/attributes/terrain-size.md")),
        "add_object" => Some(include_str!("../hover_docs/attributes/add-object.md")),
        "number_of_objects" => Some(include_str!(
            "../hover_docs/attributes/number-of-objects.md"
        )),
        "number_of_groups" => Some(include_str!("../hover_docs/attributes/number-of-groups.md")),
        "group_variance" => Some(include_str!("../hover_docs/attributes/group-variance.md")),
        "group_placement_radius" => Some(include_str!(
            "../hover_docs/attributes/group-placement-radius.md"
        )),
        "set_tight_grouping" => Some(include_str!(
            "../hover_docs/attributes/set-tight-grouping.md"
        )),
        "set_loose_grouping" => Some(include_str!(
            "../hover_docs/attributes/set-loose-grouping.md"
        )),
        "min_connected_tiles" => Some(include_str!(
            "../hover_docs/attributes/min-connected-tiles.md"
        )),
        "resource_delta" => Some(include_str!("../hover_docs/attributes/resource-delta.md")),
        "second_object" => Some(include_str!("../hover_docs/attributes/second-object.md")),
        "set_scaling_to_map_size" => Some(include_str!(
            "../hover_docs/attributes/set-scaling-to-map-size.md"
        )),
        "set_scaling_to_player_number" => Some(include_str!(
            "../hover_docs/attributes/set-scaling-to-player-number.md"
        )),
        "set_place_for_every_player" => Some(include_str!(
            "../hover_docs/attributes/set-place-for-every-player.md"
        )),
        "place_on_specific_land_id" => Some(include_str!(
            "../hover_docs/attributes/place-on-specific-land-id.md"
        )),
        "avoid_other_land_zones" => Some(include_str!(
            "../hover_docs/attributes/avoid-other-land-zones.md"
        )),
        "generate_for_first_land_only" => Some(include_str!(
            "../hover_docs/attributes/generate-for-first-land-only.md"
        )),
        "set_gaia_object_only" => Some(include_str!(
            "../hover_docs/attributes/set-gaia-object-only.md"
        )),
        "set_gaia_unconvertible" => Some(include_str!(
            "../hover_docs/attributes/set-gaia-unconvertible.md"
        )),
        "set_building_capturable" => Some(include_str!(
            "../hover_docs/attributes/set-building-capturable.md"
        )),
        "make_indestructible" => Some(include_str!(
            "../hover_docs/attributes/make-indestructible.md"
        )),
        "min_distance_to_players" => Some(include_str!(
            "../hover_docs/attributes/distance-to-players.md"
        )),
        "max_distance_to_players" => Some(include_str!(
            "../hover_docs/attributes/distance-to-players.md"
        )),
        "set_circular_placement" => Some(include_str!(
            "../hover_docs/attributes/set-circular-placement.md"
        )),
        "terrain_to_place_on" => Some(include_str!(
            "../hover_docs/attributes/terrain-to-place-on.md"
        )),
        "layer_to_place_on" => Some(include_str!(
            "../hover_docs/attributes/layer-to-place-on.md"
        )),
        "ignore_terrain_restrictions" => Some(include_str!(
            "../hover_docs/attributes/ignore-terrain-restrictions.md"
        )),
        "max_distance_to_other_zones" => Some(include_str!(
            "../hover_docs/attributes/max-distance-to-other-zones.md"
        )),
        "place_on_forest_zone" => Some(include_str!(
            "../hover_docs/attributes/place-on-forest-zone.md"
        )),
        "avoid_forest_zone" => Some(include_str!(
            "../hover_docs/attributes/avoid-forest-zone.md"
        )),
        "avoid_cliff_zone" => Some(include_str!("../hover_docs/attributes/avoid-cliff-zone.md")),
        "min_distance_to_map_edge" => Some(include_str!(
            "../hover_docs/attributes/min-distance-to-map-edge.md"
        )),
        "min_distance_group_placement" => Some(include_str!(
            "../hover_docs/attributes/min-distance-group-placement.md"
        )),
        "temp_min_distance_group_placement" => Some(include_str!(
            "../hover_docs/attributes/temp-min-distance-group-placement.md"
        )),
        "find_closest" => Some(include_str!("../hover_docs/attributes/find-closest.md")),
        "find_closest_to_map_center" => Some(include_str!(
            "../hover_docs/attributes/find-closest-to-map-center.md"
        )),
        "find_closest_to_map_edge" => Some(include_str!(
            "../hover_docs/attributes/find-closest-to-map-edge.md"
        )),
        "enable_tile_shuffling" => Some(include_str!(
            "../hover_docs/attributes/enable-tile-shuffling.md"
        )),
        "require_path" => Some(include_str!("../hover_docs/attributes/require-path.md")),
        "force_placement" => Some(include_str!("../hover_docs/attributes/force-placement.md")),
        "actor_area" => Some(include_str!("../hover_docs/attributes/actor-area.md")),
        "actor_area_radius" => Some(include_str!(
            "../hover_docs/attributes/actor-area-radius.md"
        )),
        "override_actor_radius_if_required" => Some(include_str!(
            "../hover_docs/attributes/override-actor-radius-if-required.md"
        )),
        "actor_area_to_place_in" => Some(include_str!(
            "../hover_docs/attributes/actor-area-to-place-in.md"
        )),
        "avoid_actor_area" => Some(include_str!("../hover_docs/attributes/avoid-actor-area.md")),
        "avoid_all_actor_areas" => Some(include_str!(
            "../hover_docs/attributes/avoid-all-actor-areas.md"
        )),
        "set_facet" => Some(include_str!("../hover_docs/attributes/set-facet.md")),
        "match_player_civ" => Some(include_str!("../hover_docs/attributes/match-player-civ.md")),
        // Ambiguous: Command-Attributes
        "base_terrain" => Some(include_str!(
            "../hover_docs/command-attributes/base-terrain.md"
        )),
        "base_layer" => Some(include_str!(
            "../hover_docs/command-attributes/base-layer.md"
        )),
        // Keywords
        "else" => Some(include_str!("../hover_docs/keywords/else.md")),
        "if" => Some(include_str!("../hover_docs/keywords/if.md")),
        "elseif" => Some(include_str!("../hover_docs/keywords/elseif.md")),
        "endif" => Some(include_str!("../hover_docs/keywords/endif.md")),
        "start_random" => Some(include_str!("../hover_docs/keywords/start-random.md")),
        "end_random" => Some(include_str!("../hover_docs/keywords/end-random.md")),
        "percent_chance" => Some(include_str!("../hover_docs/keywords/percent-chance.md")),
        "rnd" => Some(include_str!("../hover_docs/keywords/rnd.md")),
        "#define" => Some(include_str!("../hover_docs/keywords/define.md")),
        "#const" => Some(include_str!("../hover_docs/keywords/const.md")),
        "#include-drs" => Some(include_str!("../hover_docs/keywords/include-drs.md")),
        "#includexs" => Some(include_str!("../hover_docs/keywords/includexs.md")),
        _ => None,
    }
}

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

/// Returns a list of completions for the given text and position.
///
/// The `COMPLETABLE_TOKENS` list defines the tokens that can be completed.
/// A valid completion occurs when the prefix at the current position is
/// contained in a token from the `COMPLETABLE_TOKENS` list.
/// Comparisons are case-insensitive.
///
/// Returns `None` if the position is in a comment.
/// Returns the entire list if there is no prefix to complete.
fn get_completions(text: &str, position: Position) -> Option<Vec<CompletionItem>> {
    // No autocomplete in comments.
    let context = rms::document_context_at(text, position);
    if context.in_comment {
        return None;
    }

    // Note the empty string matches everything.
    // Converts to lowercase for case-insensitive matching.
    let completion_text = rms::extract_autocomplete_prefix(text, position)?;
    let range = Range {
        start: Position {
            line: position.line,
            character: completion_text.left as u32,
        },
        end: Position {
            line: position.line,
            character: completion_text.right as u32,
        },
    };
    let full = completion_text.token.to_lowercase();
    let mut matches: Vec<&CompletableToken> = COMPLETABLE_TOKENS
        .iter()
        .filter(|token| token.lower.contains(&full))
        .collect();
    if matches.is_empty() {
        let prefix_end =
            (position.character as usize - completion_text.left).min(completion_text.token.len());
        let prefix = completion_text.token[..prefix_end].to_lowercase();
        matches = COMPLETABLE_TOKENS
            .iter()
            .filter(|token| token.lower.contains(&prefix))
            .collect();
    }
    Some(
        matches
            .iter()
            .map(|token| CompletionItem {
                documentation: lookup_hover(token.label).map(|doc| {
                    Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: doc.to_string(),
                    })
                }),
                label: token.label.to_string(),
                kind: Some(token.kind),
                insert_text_format: token.snippet.map(|_| InsertTextFormat::SNIPPET),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range,
                    new_text: token.snippet.unwrap_or(token.label).to_string(),
                })),
                ..Default::default()
            })
            .collect(),
    )
}
