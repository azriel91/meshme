use amethyst::{
    assets::{AssetStorage, Handle, Loader, Processor, ProgressCounter},
    core::transform::TransformBundle,
    ecs::WorldExt,
    renderer::{types::DefaultBackend, RenderingBundle},
    utils::application_root_dir,
    Application, GameData, GameDataBuilder, State, StateData, StateEvent, Trans,
};

use gltf_asset::GltfAsset;
use gltf_format::GltfFormat;

mod gltf_asset;
mod gltf_format;
mod importer;

#[derive(Debug, Default)]
pub struct Example {
    progress_counter: Option<ProgressCounter>,
}

impl Example {
    fn print_meshes(gltf_asset: &GltfAsset) {
        gltf_asset
            .0
            .meshes()
            .flat_map(|mesh| mesh.primitives())
            .map(|primitive| primitive.bounding_box())
            .for_each(|bounding_box| println!("{:?}", bounding_box));
    }
}

/// Counter so we don't wait indefinitely.
#[derive(Debug)]
pub struct Counter(u32);

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Example {
    fn on_start(&mut self, mut state_data: StateData<'_, GameData<'a, 'b>>) {
        let world = &mut state_data.world;

        let mut progress_counter = ProgressCounter::new();

        // Request the GLTF data to be loaded.
        let gltf_asset_handle = {
            let loader = world.read_resource::<Loader>();

            let gltf_asset_storage = world.read_resource::<AssetStorage<GltfAsset>>();
            loader.load(
                "puffy.gltf",
                GltfFormat,
                &mut progress_counter,
                &gltf_asset_storage,
            )
        };

        self.progress_counter = Some(progress_counter);
        world.insert(gltf_asset_handle);
        world.insert(Counter(0));
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        // Run the dispatcher, which loads the `GltfAsset`.
        data.data.update(&data.world);

        if let Some(progress_counter) = self.progress_counter.as_ref() {
            if progress_counter.is_complete() {
                let gltf_asset_handle = data.world.read_resource::<Handle<GltfAsset>>().clone();
                let gltf_asset_storage = data.world.read_resource::<AssetStorage<GltfAsset>>();
                let gltf_asset = gltf_asset_storage
                    .get(&gltf_asset_handle)
                    .expect("`GltfAsset` should be loaded, so this should be some.");

                // Do something with gltf_asset
                log::info!("`GltfAsset` loaded!");

                Self::print_meshes(gltf_asset);

                return Trans::Quit;
            }
        }

        let mut counter = data.world.write_resource::<Counter>();
        counter.0 += 1;
        if counter.0 == 1000 {
            let message = "`GltfAsset` not loaded after 1000 iterations. Check:\n\
            * asset path\n\
            * systems needed to load GLTF -- GltfSceneLoaderSystemDesc, animation stuff?\n";
            log::error!("{}", message);
            eprintln!("{}", message);
            return Trans::Quit;
        }

        Trans::None
    }
}

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with(Processor::<GltfAsset>::new(), "gltf_asset_processor", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new())?;

    let mut game = Application::build(assets_dir, Example::default())?.build(game_data)?;
    game.run();
    Ok(())
}
