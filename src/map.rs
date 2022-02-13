use rand::Rng;

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<u32>
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut tiles = vec![0; (width * height) as usize];
        let mut rng = rand::thread_rng();
        for i in 0..width*height {
            if rng.gen_range(0..10) == 1 {
                tiles[i as usize] = 1;
            }
        }
        Map {
            width: width,
            height: height,
            tiles: tiles
        }
    }

    pub fn xy_idx(&self, x: u32, y: u32) -> u32 {
        y * self.width + x
    }
}
