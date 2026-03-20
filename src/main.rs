//! Aoe2 RMS Language Server

mod rms;

use std::collections::HashMap;

use tokio::sync::RwLock;
use tower_lsp::Client;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover,
    HoverContents, HoverParams, HoverProviderCapability, MarkupContent, MarkupKind, Position,
    PositionEncodingKind, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    Url,
};
use tower_lsp::{
    LanguageServer, LspService, Server,
    lsp_types::{InitializeParams, InitializeResult, InitializedParams, MessageType},
};

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
}

#[tokio::main]
async fn main() {
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
    let token = extract_token(text, position)?;
    lookup_hover(token, context.in_block)
}

/// Extracts the token at the given position from the text.
/// Returns `Some(token)` if the position is within text.
/// Otherwise returns `None` if the position is within whitespace.
fn extract_token(text: &str, position: Position) -> Option<&str> {
    let line = text.split("\n").nth(position.line as usize)?;
    let col = position.character as usize;
    let right = line[col..]
        .find(char::is_whitespace)
        .map(|i| i + col)
        .unwrap_or(line.len());
    if right == col {
        return None;
    }
    let left = line[..col]
        .rfind(char::is_whitespace)
        .map(|i| i + 1)
        .unwrap_or(0);
    debug_assert_ne!(
        left, right,
        "right > left because the first character is not whitespace"
    );
    Some(&line[left..right])
}

/// Returns the hover content for the given token, or `None` if the token does
/// not have hover data. `in_block` is used to determine if the token is inside
/// a block, which affects which hover content is returned for `base_terrain`
/// and `base_layer`.
fn lookup_hover(token: &str, in_block: bool) -> Option<&'static str> {
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
        // Ambiguous
        "base_terrain" => lookup_base_terrain_hover(in_block),
        "base_layer" => lookup_base_layer_hover(in_block),
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

/// Returns the hover content for `base_terrain`.
fn lookup_base_terrain_hover(in_block: bool) -> Option<&'static str> {
    if in_block {
        Some(include_str!("../hover_docs/attributes/base-terrain.md"))
    } else {
        Some(include_str!("../hover_docs/commands/base-terrain.md"))
    }
}

/// Returns the hover content for `base_layer`.
fn lookup_base_layer_hover(in_block: bool) -> Option<&'static str> {
    if in_block {
        Some(include_str!("../hover_docs/attributes/base-layer.md"))
    } else {
        Some(include_str!("../hover_docs/commands/base-layer.md"))
    }
}
