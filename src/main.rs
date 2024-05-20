use three_d::*;
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]

async fn main() {
    run().await;
}

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "AMOGUS".to_string(),
        max_size: Some((1280, 720)),
         ..Default::default()
    })
    .unwrap();
    let context = window.gl();
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(2800.0, 240.0, 1700.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10000.0,
    );
    let mut control = FirstPersonControl::new(0.1);

    let mut loaded = if let Ok(loaded) = three_d_asset::io::load_async(&[
        "/home/teo/cspmakeagame/assets/space.hdr"]).await
    {
        loaded
    } else {
        println!("loading from web");
        three_d_asset::io::load_async(&[
            "http://localhost:8000/space.hdr",])
            .await
            .expect("this is an error message")
    };

    let skybox = Skybox::new_from_equirectangular(&context, &loaded.deserialize("hdr").unwrap());
    let light = AmbientLight::new_with_environment(&context, 0.7, Srgba::WHITE, skybox.texture());
    let mut gui = GUI::new(&context);

    window.render_loop(move |mut frame_input| {
        let panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                
                /*SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("FLAPPY BIRD");
                });
                panel_width = gui_context.used_rect().width();*/
            },
        );
        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y:0,
            width: 1280,
            height: 720,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, skybox.into_iter(), &[&light])
            .write(|| gui.render())
            .unwrap();
        FrameOutput::default()
    });
}
