use amethyst::{
    assets::{Format, FormatValue, Source},
    error::Error,
};
use std::sync::Arc;

use crate::{gltf_asset::GltfAsset, importer::import};

/// Format for loading from `.mylang` files.
#[derive(Clone, Copy, Debug, Default)]
pub struct GltfFormat;

impl Format<GltfAsset> for GltfFormat {
    fn name(&self) -> &'static str {
        "GLTFScene"
    }

    fn import(
        &self,
        name: String,
        source: Arc<dyn Source>,
        _create_reload: Option<Box<dyn Format<GltfAsset>>>,
    ) -> Result<FormatValue<GltfAsset>, Error> {
        import(source.clone(), name).map(|(gltf, _buffers)| {
            // load_data(&gltf, &buffers, options, source, name).map_err(Into::into)

            FormatValue::data(GltfAsset(gltf))
        })
    }
}
