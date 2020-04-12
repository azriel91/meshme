use amethyst::{
    assets::{Asset, Handle},
    ecs::VecStorage,
};
use gltf::Gltf; // amethyst_gltf should re-export this.

/// Custom asset representing GLTF data.
#[derive(Clone, Debug)]
pub struct GltfAsset(pub Gltf);

/// A handle to a `GltfAsset`.
pub type GltfAssetHandle = Handle<GltfAsset>;

impl Asset for GltfAsset {
    const NAME: &'static str = concat!(module_path!(), stringify!(GltfAsset));
    type Data = Self;
    type HandleStorage = VecStorage<GltfAssetHandle>;
}
