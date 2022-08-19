use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(path = "sprites/player.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/archer.png")]
    pub archer: Handle<Image>,
    #[asset(path = "sprites/alchemist.png")]
    pub alchemist: Handle<Image>,
    #[asset(path = "sprites/cyclops.png")]
    pub cyclops: Handle<Image>,
    #[asset(path = "sprites/lobster.png")]
    pub lobster: Handle<Image>,
    #[asset(path = "sprites/dwarf.png")]
    pub dwarf: Handle<Image>,
    #[asset(path = "sprites/ghost.png")]
    pub ghost: Handle<Image>,
    #[asset(path = "sprites/knight.png")]
    pub knight: Handle<Image>,
    #[asset(path = "sprites/evil_wizard.png")]
    pub evil_wizard: Handle<Image>,
    #[asset(path = "sprites/bat.png")]
    pub bat: Handle<Image>,
    #[asset(path = "sprites/cactus.png")]
    pub cactus: Handle<Image>,
    #[asset(path = "sprites/rat1.png")]
    pub rat1: Handle<Image>,
    #[asset(path = "sprites/rat2.png")]
    pub rat2: Handle<Image>,
    #[asset(path = "sprites/wizard.png")]
    pub wizard: Handle<Image>,
    #[asset(path = "sprites/spider.png")]
    pub spider: Handle<Image>,
    #[asset(path = "sprites/background.png")]
    pub background: Handle<Image>,
}
#[derive(Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);

#[derive(Deref, DerefMut)]
pub struct EnemiesCount(pub u32);
