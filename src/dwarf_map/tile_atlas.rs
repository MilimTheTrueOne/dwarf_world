use bevy::prelude::*;

#[derive(Resource)]
pub struct TileAtlas {
    handle: Handle<Image>,
    image_tile_count: Vec2,
}

impl TileAtlas {
    pub fn get_uvs(&self, index: u32) -> [Vec2; 4] {
        let pos = Vec2::new(
            index as f32 % self.image_tile_count.y,
            (index / self.image_tile_count.y as u32) as f32,
        );

        let u = [
            Vec2::new(pos.x, pos.y + 1.0) / self.image_tile_count,
            Vec2::new(pos.x + 1.0, pos.y + 1.0) / self.image_tile_count,
            Vec2::new(pos.x + 1.0, pos.y) / self.image_tile_count,
            pos / self.image_tile_count,
        ];

        [
            u[0] + Vec2::new(0.05, -0.05),
            u[1] + Vec2::splat(-0.05),
            u[2] + Vec2::new(-0.05, 0.05),
            u[3] + Vec2::splat(0.05),
        ]
    }

    pub fn get_handle(&self) -> Handle<Image> {
        self.handle.clone()
    }
}

pub fn init_atlas(mut commands: Commands, asset_server: Res<AssetServer>) {
    let atlas = TileAtlas {
        handle: asset_server.load("tiles.png"),
        image_tile_count: Vec2::splat(2.0),
    };
    commands.insert_resource(atlas)
}

#[cfg(test)]
mod test {
    use bevy::math::Vec2;

    use super::TileAtlas;

    #[test]
    fn atlas() {
        let atlas = TileAtlas {
            handle: Default::default(),
            image_tile_count: Vec2::splat(2.0),
        };

        assert_eq!(atlas.get_uvs(0)[1].y, 0.5)
    }
}
