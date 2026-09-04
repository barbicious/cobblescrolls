pub const TOTAL_VERTICES: usize = 180;

macro_rules! uv {
    ($x:expr, $y:expr) => {
        (
            ($x * TileType::TEXTURE_WIDTH) / crate::graphics::texture_atlas::TextureAtlas::WIDTH,
            ($y * TileType::TEXTURE_HEIGHT) / crate::graphics::texture_atlas::TextureAtlas::HEIGHT,
        )
    };
}

#[derive(Copy, Clone)]
pub enum TileType {
    Air,
    Grass,
    Stone,
}

impl TileType {
    const TEXTURE_WIDTH: f32 = 8.0;
    const TEXTURE_HEIGHT: f32 = 8.0;

    fn uv(&self) -> (f32, f32) {
        match self {
            TileType::Air => panic!("Attempted to get uv coords for air!"),
            TileType::Grass => uv!(0.0, 0.0),
            TileType::Stone => uv!(1.0, 0.0),
        }
    }

    pub fn vertices(&self, face: Face, x: i32, y: i32, z: i32) -> [f32; 30] {
        let (u, v) = self.uv();

        let (mut u_full, mut v_full) = uv!(1.0, 1.0);
        u_full += u;
        v_full += v;

        let x = x as f32;
        let y = y as f32;
        let z = z as f32;

        let world_x = x + 1.0;
        let world_y = y + 1.0;
        let world_z = z + 1.0;

        match face {
            Face::Back => [
                x, world_y, z, u_full, v_full, world_x, world_y, z, u, v_full, world_x, y, z, u, v,
                world_x, y, z, u, v, x, y, z, u_full, v, x, world_y, z, u_full, v_full,
            ],
            Face::Front => [
                x, y, world_z, u, v, world_x, y, world_z, u_full, v, world_x, world_y, world_z,
                u_full, v_full, world_x, world_y, world_z, u_full, v_full, x, world_y, world_z, u,
                v_full, x, y, world_z, u, v,
            ],
            Face::Left => [
                x, y, z, u, v, x, y, world_z, u_full, v, x, world_y, world_z, u_full, v_full, x,
                world_y, world_z, u_full, v_full, x, world_y, z, u, v_full, x, y, z, u, v,
            ],
            Face::Right => [
                world_x, y, world_z, u, v, world_x, y, z, u_full, v, world_x, world_y, z, u_full,
                v_full, world_x, world_y, z, u_full, v_full, world_x, world_y, world_z, u, v_full,
                world_x, y, world_z, u, v,
            ],
            Face::Top => [
                x, world_y, z, u, v, x, world_y, world_z, u_full, v, world_x, world_y, world_z,
                u_full, v_full, world_x, world_y, world_z, u_full, v_full, world_x, world_y, z, u,
                v_full, x, world_y, z, u, v,
            ],
            Face::Bottom => [
                x, y, world_z, u, v, x, y, z, u_full, v, world_x, y, z, u_full, v_full, world_x, y,
                z, u_full, v_full, world_x, y, world_z, u, v_full, x, y, world_z, u, v,
            ],
        }
    }
}

pub enum Face {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Face {
    pub const VERTICES: f32 = 30.0;
}