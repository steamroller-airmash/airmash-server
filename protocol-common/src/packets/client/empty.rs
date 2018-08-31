/// TODO: Figure out what this is for.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Ack;

/// Request a detailed score packet from the server.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScoreDetailed;
