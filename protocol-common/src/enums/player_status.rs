/// Flag for indicating whether a player is
/// alive or dead.
///
/// This is used in the following packets:
/// - [`Login`] (specifically [`LoginPlayer`])
/// - [`PlayerNew`]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Alive = 0,
    Dead = 1,
}
