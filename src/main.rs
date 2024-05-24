use three_d::*;
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]

async fn main() {
    run().await;
}

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "".to_string(),
        max_size: Some((1280, 720)),
         ..Default::default()
    })
    .unwrap();
    let context = window.gl();
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-600.0, 600.0, 600.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    let mut control = OrbitControl::new(
        *camera.target(),
        0.5*camera.target().distance(*camera.position()),
        5.0*camera.target().distance(*camera.position()),
    );

    let mut loaded = if let Ok(loaded) = three_d_asset::io::load_async(&[
        "/home/teo/cspmakeagame/assets/space.hdr", "/home/teo/cspmakeagame/assets/iss.obj"]).await
    {
        loaded
    } else {
        println!("loading from web");
        three_d_asset::io::load_async(&[
            "http://localhost/space.hdr",
            "http://localhost/iss.obj",])
            .await
            .expect("this is an error message")
    };
    
    let cpu_model: CpuModel = loaded.deserialize("iss.obj").unwrap();
    let mut model = Vec::new();
    let scale = Mat4::from_scale(10.0);
    
    let mut iss = Model::<PhysicalMaterial>::new(&context,
        &loaded.deserialize("iss.obj").unwrap())
            .unwrap();
    iss.iter_mut().for_each(|m| {
        m.material.render_states.cull = Cull::Back;
        m.set_transformation(Mat4::from_angle_x(degrees(-90.0)));
    });
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
            |gui_context| {},
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
            .write(|| {
                for object in model
                    .iter()
                    .flatten()
                    .chain(&iss)
                    .filter(|o| camera.in_frustum(&o.aabb()))
                {
                    object.render(&camera, &[&light]);
                }
                gui.render()
            })
            .unwrap();

        FrameOutput::default()
    });
}
