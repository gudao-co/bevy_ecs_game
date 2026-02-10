use bevy_ecs::prelude::*;
use bevy_ecs_rpc::core::*;
use bevy_ecs_rpc::mem::*;

#[derive(Resource)]
pub struct Time {
    dt: f32,
}

pub struct Game {
    pub world: World,
    pub scheduler: Schedule,
    buf: Vec<u8>,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        let scheduler = Schedule::default();

        world.insert_resource(RpcMem::new());
        world.insert_resource(Time { dt: 0.0 });

        Self {
            world: world,
            scheduler: scheduler,
            buf: Vec::new(),
        }
    }

    pub fn snapshot(&mut self) -> &Vec<u8> {
        self.world.resource_scope(|_, mut rpc: Mut<RpcMem>| {
            self.buf.clear();
            self.buf.extend_from_slice(rpc.snapshot());
        });
        &self.buf
    }

    pub fn update(&mut self, dt: f32, invoke: &[u8]) -> &Vec<u8> {
        self.world.resource_scope(|_, mut time: Mut<Time>| {
            time.dt = dt;
        });

        self.world.resource_scope(|world, mut rpc: Mut<RpcMem>| {
            rpc.clear();
            rpc.invoke(invoke, world);
        });

        self.scheduler.run(&mut self.world);

        self.world.resource_scope(|_, rpc: Mut<RpcMem>| {
            self.buf.extend_from_slice(rpc.data());
        });
        &self.buf
    }
}
