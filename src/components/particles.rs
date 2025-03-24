use dioxus::prelude::*;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    window, CanvasRenderingContext2d, HtmlCanvasElement,
};

struct Particle {
    x: f64,
    y: f64,
    size: f64,
    speed_x: f64,
    speed_y: f64,
    color: String,
}

const PARTICLE_COUNT: usize = 100;

#[component]
pub fn Particles() -> Element {
    use_effect(move || {
        init_canvas();
    });
    rsx! {
        canvas { id: "background-particles-canvas" }
    }
}

fn init_canvas() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("background-particles-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // set canvas size
    resize_canvas(&canvas);

    let _particles = create_particles(&canvas);

    let particles = Rc::new(RefCell::new(create_particles(&canvas)));
    let canvas = Rc::new(canvas);
    let ctx = Rc::new(ctx);

    // Start animation loop
    animate(ctx.clone(), canvas.clone(), particles.clone());
}

fn animate(
    ctx: Rc<CanvasRenderingContext2d>,
    canvas: Rc<HtmlCanvasElement>,
    particles: Rc<RefCell<Vec<Particle>>>,
) {
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    // Update and draw particles
    let mut particles_borrow = particles.borrow_mut();
    for particle in particles_borrow.iter_mut() {
        // Move particles
        particle.x += particle.speed_x;
        particle.y += particle.speed_y;

        // bounce off edges
        if particle.x > canvas.width() as f64 || particle.x < 0.0 {
            particle.speed_x = -particle.speed_x;
        }
        if particle.y > canvas.height() as f64 || particle.y < 0.0 {
            particle.speed_y = -particle.speed_y;
        }

        // Draw particle
        ctx.begin_path();
        ctx.arc(
            particle.x,
            particle.y,
            particle.size,
            0.0,
            2.0 * f64::consts::PI,
        )
        .unwrap();
        ctx.set_fill_style_str(&particle.color);
        ctx.fill();
    }

    // request next frame
    let ctx_clone = ctx.clone();
    let canvas_clone = canvas.clone();
    let particles_clone = particles.clone();

    request_animation_frame(move || {
        animate(ctx_clone, canvas_clone, particles_clone);
    });
}

fn resize_canvas(canvas: &HtmlCanvasElement) {
    let window = window().unwrap();
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
}

fn create_particles(canvas: &HtmlCanvasElement) -> Vec<Particle> {
    let mut particles = Vec::with_capacity(PARTICLE_COUNT);
    let mut rng = oorandom::Rand64::new(4);

    for _ in 0..PARTICLE_COUNT {
        let theme = if let Some(document) = window().unwrap().document() {
            if let Some(element) = document.document_element() {
                element
                    .get_attribute("data-theme")
                    .unwrap_or_else(|| String::from("mocha"))
            } else {
                String::from("mocha")
            }
        } else {
            String::from("mocha")
        };
        let accent_color;
        match theme.as_str() {
            "mocha" => accent_color = "#89b4fa",
            "macchiato" => accent_color = "#8aadf4",
            "latte" => accent_color = "#8839ef",
            "rosepine" => accent_color = "#9ccfd8",
            _ => accent_color = "#89b4fa",
        }
        particles.push(Particle {
            x: rng.rand_float() * canvas.width() as f64,
            y: rng.rand_float() * canvas.height() as f64,
            size: rng.rand_float() * 5.0 + 1.0,
            speed_x: (rng.rand_float() - 0.5) * 0.5,
            speed_y: (rng.rand_float() - 0.5) * 0.5,
            color: accent_color.to_string(),
        });
    }

    particles
}

fn request_animation_frame<F>(f: F)
where
    F: FnOnce() + 'static,
{
    let window = window().unwrap();
    let closure = Closure::once(f);
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}
