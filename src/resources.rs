use bevy::{audio::AudioSink, prelude::*};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/player-sheet.png")]
    pub player: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/archer-sheet.png")]
    pub archer: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/alchemist-sheet.png")]
    pub alchemist: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/cyclops-sheet.png")]
    pub cyclops: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/lobster-sheet.png")]
    pub lobster: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/dwarf-sheet.png")]
    pub dwarf: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 1))]
    #[asset(path = "sprites/ghost.png")]
    pub ghost: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/knight-sheet.png")]
    pub knight: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/evil_wizard-sheet.png")]
    pub evil_wizard: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/bat-sheet.png")]
    pub bat: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/rat1-sheet.png")]
    pub rat: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/wizard-sheet.png")]
    pub wizard: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/spider-sheet.png")]
    pub spider: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 1))]
    #[asset(path = "sprites/fireball-sheet.png")]
    pub fireball: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 1))]
    #[asset(path = "sprites/arrow.png")]
    pub arrow: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 1))]
    #[asset(path = "sprites/slash-sheet.png")]
    pub slash: Handle<TextureAtlas>,
    #[asset(path = "sprites/background.png")]
    pub background: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct Sounds {
    #[asset(path = "sounds/arrowsound.wav")]
    pub arrow: Handle<AudioSource>,
    #[asset(path = "sounds/enemyhit.wav")]
    pub enemyhit: Handle<AudioSource>,
    #[asset(path = "sounds/fireballsound.wav")]
    pub fireball: Handle<AudioSource>,
    #[asset(path = "sounds/hitsound.wav")]
    pub player_hit: Handle<AudioSource>,
    #[asset(path = "sounds/slashsound.wav")]
    pub slash: Handle<AudioSource>,
    #[asset(path = "sounds/menumusic.wav")]
    pub menu: Handle<AudioSource>,
    #[asset(path = "sounds/gamemusic.wav")]
    pub gamemusic: Handle<AudioSource>,
}

#[derive(Deref, DerefMut)]
pub struct DraggingEntity(pub Option<Entity>);

#[derive(Deref, DerefMut)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Deref, DerefMut)]
pub struct AllySpawnTimer(pub Timer);

#[derive(Deref, DerefMut)]
pub struct EnemiesCount(pub u32);

#[derive(Deref, DerefMut)]
pub struct AllyCount(pub u32);
