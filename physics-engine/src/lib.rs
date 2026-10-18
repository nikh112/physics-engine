use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub mass: f64,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub ax: f64,
    pub ay: f64,
    pub radius: f64,
}

#[wasm_bindgen]

impl Body {
    #[wasm_bindgen(constructor)]
    pub fn new(mass: f64, x: f64, y: f64, vx: f64, vy: f64,radius: f64) -> Body { // initial velocity, but acceleration will be zero
        Body { mass, x, y, vx, vy, ax: 0.0, ay: 0.0, radius}
    }
}

#[wasm_bindgen]
pub struct Simulation {
    bodies: Vec<Body>
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Simulation {
        Simulation { bodies: Vec::new() }
    }

    pub fn bodies_ptr(&self) -> *const Body {
        self.bodies.as_ptr()
    }

    pub fn num_bodies(&self) -> usize {
        self.bodies.len()
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn  tick(&mut self, dt: f64) {
        let n = self.bodies.len(); // amount of bodies

        // update all bodies positions
        for i in 0..n {
            let b = &mut self.bodies[i]; // must put &mut to make mutable
            b.x += b.vx * dt + 0.5 * b.ax * dt * dt;
            b.y += b.vy * dt + 0.5 * b.ay * dt * dt;
        }

        // store old accelerations to calculate the new velocity
        let mut old_ax = vec![0.0; n]; // size being n for n bodies
        let mut old_ay = vec![0.0 ; n]; // size being n for n bodies
        for i in 0..n {
            old_ax[i] = self.bodies[i].ax;
            old_ay[i] = self.bodies[i].ay;

            self.bodies[i].ax = 0.0;
            self.bodies[i].ay = 0.0;
        }

        // Calculate new forces
        const G: f64 = 6.67430e-11;
        const S: f64 = 1e-4; // softening to prevent 0 dist for close bodies

        for i in 0..n {
            for j in (i + 1)..n { // we want every individual pair of bodies, i and i + 1 is one pair
                // calculate the x and y distance between the two bodies
                let dx = self.bodies[j].x - self.bodies[i].x;
                let dy = self.bodies[j].y - self.bodies[i].y;
                let dist_sq = dx * dx + dy * dy;
                
                let dist = (dist_sq + S).sqrt(); // distance between the bodies, radius
                let force = G * self.bodies[i].mass * self.bodies[j].mass / dist_sq;

                let fx = force * dx / dist;
                let fy = force * dy / dist;

                // editing accelerations based on force applied (A)
                self.bodies[i].ax += fx / self.bodies[i].mass;
                self.bodies[i].ay += fy / self.bodies[i].mass; 

                // editing accelerations based on OPPOSITE force applied (B)
                self.bodies[j].ax -= fx / self.bodies[j].mass;
                self.bodies[j].ay -= fy / self.bodies[j].mass; 
            }
        }

        for i in 0..n {
            let b = &mut self.bodies[i];

            b.vx += 0.5 * (old_ax[i] + b.ax) * dt;
            b.vy += 0.5 * (old_ay[i] + b.ay) * dt;
        }
    }


}
