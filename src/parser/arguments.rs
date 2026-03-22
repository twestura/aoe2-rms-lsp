//! Arguments for RMS Commands and Attributes.

/// The kind of an argument for an instruction.
/// An Argument may be a label (for an if statement) or allow
/// a set of constant kinds.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArgumentKind {
    /// A label for an if statement.
    Label,
    /// A set of constant kinds.
    Constant(Vec<ConstantKind>),
}

/// The kind of a constant for an instruction.
/// The kind is used to determine which constants to display for
/// autocompletion.
///
/// Note the lists are not exhaustive. Not all values have a predefined
/// constant name. Map authors must define their own constants for many
/// values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstantKind {
    /// A terrain constant.
    Terrain,

    /// An object constant.
    Object,

    /// A civilization constant.
    Civilization,

    /// Names of many built-in maps.
    MapType,

    /// The type of effect (e.g. set, add, mul) used by effect amount and
    /// effect percent.
    Effect,

    /// Includes the EffectType, AttributeType, and ModifyTech
    /// constants listed in the RMS documentation.
    AttributeType,

    /// Player resources used by effects to modify game data.
    /// They control player-specific data such as starting resources.
    ResourceType,

    /// Technology types used by effects.
    /// All technologies must be user-defined, there are no pre-defined
    /// constants.
    TechnologyType,

    /// Classes that target a wide range of units with similar characteristics.
    UnitClass,

    /// The type of cliff.
    CliffType,

    /// Color correction season.
    SeasonType,

    /// Water theme.
    WaterDefinition,

    /// Used by assign_to to control team assignment of players.
    AssignType,

    /// A one-off constant: DATA_CIV_NAME_ID.
    /// Used with the SET_PLAYER_DATA effect type.
    PlayerData,

    /// Can be used with effect amount and ATTR_NAME_ID to rename objects.
    /// Not many are defined, and for the most part the renaming requires
    /// looking up string IDs.
    LanguageStringId,

    /// Any other constant without a predefined list of options.
    /// Includes numbers, filepaths, and identifiers for constants and labels.
    Other,
}

/// The kind of a label for an instruction.
/// These labels are defined dynamically when the game is launched.
/// They allow the map to access game settings, such as the game mode or
/// map size, that are selected in the lobby.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabelKind {
    /// Type of game selected in the dropdown box, such as Random Map or
    /// Regicide.
    GameMode,

    /// The size of the map. These labels are the classic way of querying the
    /// map size within the script.
    MapSize,

    /// Additional labels for map sizes. Includes more size options than
    /// MapSize.
    MapSizeModern,

    /// The amount of Food, Wood, Stone, and Gold a player starts with.
    /// Can also set the resources to be infinite.
    StartingResources,

    /// The starting age of the game.
    StartingAge,

    /// Additional checkboxes for toggling game options.
    LobbyCheckboxes,

    /// The number of players in the game.
    PlayerCount,

    /// The number of teams in the game. Teams are counted only if they have at
    /// least two players.
    TeamCount,

    /// The number of players on each team.
    TeamSize,

    /// Whether a specific player is on a specific team.
    PlayerInTeam,

    /// Whether specific game versions or expansions are available.
    /// Other versions can be checked for by using constants available only in
    /// the specific version or expansion.
    GameVersion,
}
