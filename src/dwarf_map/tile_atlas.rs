use bevy::prelude::*;

#[derive(Resource)]
pub struct TileAtlas {
    pub image: Handle<Image>,
    pub material: Handle<StandardMaterial>,
    pub layout: TextureAtlasLayout,
}

impl TileAtlas {
    pub fn get_uvs(&self, index: usize) -> [Vec2; 4] {
        let rect = self.layout.textures[index];

        let min = rect.min + Vec2::splat(0.05);
        let max = rect.max - Vec2::splat(0.05);

        [
            min / self.layout.size,
            Vec2::new(min.x, max.y) / self.layout.size,
            max / self.layout.size,
            Vec2::new(max.x, min.y) / self.layout.size,
        ]
    }
}
