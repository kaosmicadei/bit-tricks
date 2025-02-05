#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum CellValue {
    Empty = 0,
    Cross = 1,
    Circle = 2,
}

#[derive(Debug, Clone, Copy)]
struct GameState {
    state: u16,
}

impl GameState {
    fn new() -> Self {
        Self { state: 0u16 }
    }

    fn get(&self, cell: u32) -> u16 {
        (self.state / 3u16.pow(cell)) % 3
    }

    fn set(&self, cell: u32, value: CellValue) -> Self {
        let old_value = self.get(cell);
        let new_state = (value as u16).wrapping_sub(old_value)  // The wrapping_* operartions to avoid overflow.
                                           .wrapping_mul(3u16.pow(cell))
                                           .wrapping_add(self.state);

        Self { state: new_state }
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06x}", self.state)
    }
}

fn main() {
    use CellValue::*;

    let moves = [
        (0, Cross),
        (2, Circle),
        (4, Cross),
        (7, Circle),
        (7, Empty),
        (8, Circle),
    ];

    let game = moves.iter()
                               .fold(
                                    GameState::new(),
                                    |acc, m| acc.set(m.0, m.1)
                                );

    println!("{}", game);
    // 0x33a6 = 13222
}
