#![feature(test)]

extern crate airmash_server;
extern crate rand;
extern crate specs;
extern crate test;

use test::Bencher;

use specs::Entity;

use airmash_server::consts::TERRAIN;
use airmash_server::types::collision::*;
use airmash_server::Position;

use std::mem;

const ZEROS: (u32, u32) = (0, 0);
const ONES: (u32, u32) = (!0, !0);

fn generate_circles() -> Vec<HitCircle> {
	let mut circles = vec![];

	for _ in 0..16000 {
		let x: f32 = rand::random::<f32>() * 32768.0;
		let y: f32 = rand::random::<f32>() * 16384.0;
		let r: f32 = rand::random::<f32>() * 35.0;

		circles.push(HitCircle {
			pos: Position::new(x, y),
			rad: r.into(),
			layer: 1,
			// Ent is POD, and there's no way to
			// construct it without setting up all
			// of specs, this should be ok
			ent: unsafe { mem::transmute(ONES) },
		})
	}

	circles
}

#[bench]
fn terrain_collision(b: &mut Bencher) {
	let ent: Entity = unsafe { mem::transmute(ZEROS) };
	let circles = generate_circles();
	let terrain = Terrain::with_entity(TERRAIN.iter(), ent);

	b.iter(move || {
		let mut vec = vec![];

		terrain.collide(circles.iter().cloned(), &mut vec);

		vec
	})
}
