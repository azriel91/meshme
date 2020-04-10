use amethyst::{
    core::transform::TransformBundle, gltf::GltfSceneLoaderSystemDesc, utils::application_root_dir,
    Application, GameData, GameDataBuilder, State, StateData, StateEvent, Trans,
};

#[derive(Debug)]
pub struct Example;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Example {
    fn on_start(&mut self, mut _state_data: StateData<'_, GameData<'a, 'b>>) {
        // Request the GLTF data to be loaded.
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        // Run the dispatcher.
        data.data.update(&data.world);

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
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::build(assets_dir, Example)?.build(game_data)?;
    game.run();
    Ok(())
}
