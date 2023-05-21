#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Tag {
    Boss,
    Cosmetic,
    Expansion,
    Gameplay,
    Library,
    Utility,
}

impl From<Tag> for hk_modlinks::Tag {
    fn from(value: Tag) -> Self {
        match value {
            Tag::Boss => hk_modlinks::Tag::Boss,
            Tag::Cosmetic => hk_modlinks::Tag::Cosmetic,
            Tag::Expansion => hk_modlinks::Tag::Expansion,
            Tag::Gameplay => hk_modlinks::Tag::Gameplay,
            Tag::Library => hk_modlinks::Tag::Library,
            Tag::Utility => hk_modlinks::Tag::Utility,
        }
    }
}
