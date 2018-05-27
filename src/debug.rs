
use specs::*;
use types::*;

macro_rules! define_print {
    ( $($component:ty),* ) => {
        /// Print all the components that are associated with an entity.
        fn print_entity(world: &World, entity: Entity) {
            println!("Entity {}:", entity.id());
            $( 
                if let Some(component) = world.read_storage::<$component>().get(entity) {
                    println!(" - {}: {:?}", stringify!($component), component);
                }
            )*
        }
    }
}

define_print!(
	//Position,
	//Rotation,
	//Speed,
	//Accel,
	//KeyState,
	//Upgrades,
	//Energy,
	//Health,
	Plane
	
);

pub fn print_all_entities(world: &World) {
	let ref conns = world.read_resource::<Connections>();
	
	for conn in conns.iter() {
		if let Some(player) = conn.player {
			if conn.ty == ConnectionType::Primary {
				print_entity(world, player);
			}
		}
	}
}
