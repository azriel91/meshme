use amethyst::{
    animation::AnimationBundle,
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    core::transform::{Transform, TransformBundle},
    ecs::WorldExt,
    gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystemDesc},
    renderer::{types::DefaultBackend, RenderingBundle},
    utils::application_root_dir,
    Application, GameData, GameDataBuilder, State, StateData, StateEvent, Trans,
};

#[derive(Debug, Default)]
pub struct Example {
    progress_counter: Option<ProgressCounter>,
}

/// Counter so we don't wait indefinitely.
#[derive(Debug)]
pub struct Counter(u32);

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Example {
    fn on_start(&mut self, mut state_data: StateData<'_, GameData<'a, 'b>>) {
        let world = &mut state_data.world;

        let mut progress_counter = ProgressCounter::new();

        // Request the GLTF data to be loaded.
        let gltf_prefab_handle = {
            let loader = world.read_resource::<Loader>();

            // GltfSceneAsset is type alias for Prefab<GltfPrefab>
            let gltf_prefab_storage = world.read_resource::<AssetStorage<GltfSceneAsset>>();
            loader.load(
                "puffy.gltf",
                GltfSceneFormat::default(),
                &mut progress_counter,
                &gltf_prefab_storage,
            )
        };

        self.progress_counter = Some(progress_counter);
        world.insert(gltf_prefab_handle);
        world.insert(Counter(0));
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        // Run the dispatcher, which loads the GLTF scene.
        data.data.update(&data.world);

        if let Some(progress_counter) = self.progress_counter.as_ref() {
            if progress_counter.is_complete() {
                let gltf_prefab_handle =
                    data.world.read_resource::<Handle<GltfSceneAsset>>().clone();
                let gltf_prefab_storage =
                    data.world.read_resource::<AssetStorage<GltfSceneAsset>>();
                let _gltf_prefab = gltf_prefab_storage
                    .get(&gltf_prefab_handle)
                    .expect("GLTF scene should be loaded, so this should be some.");

                // Do something with gltf_prefab
                println!("GLTF scene loaded!");
                return Trans::Quit;
            }
        }

        let mut counter = data.world.write_resource::<Counter>();
        counter.0 += 1;
        if counter.0 == 1000 {
            let message = "GLTF scene not loaded after 1000 iterations. Check:\n\
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
        .with_system_desc(GltfSceneLoaderSystemDesc::default(), "gltf_loader", &[])
        .with_bundle(
            AnimationBundle::<usize, Transform>::new("animation_control", "sampler_interpolation")
                .with_dep(&["gltf_loader"]),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new())?;

    let mut game = Application::build(assets_dir, Example::default())?.build(game_data)?;
    game.run();
    Ok(())
}
