use AdvancedComputerGraphic_rs::config::create_scene;

fn main() {
	let mut scene = create_scene("config/hw1_input.txt");
    scene.display_scene().unwrap();
    scene.camera.unwrap().to_ppm("output.ppm");
}
