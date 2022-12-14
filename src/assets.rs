use crate::sprite::Sprite;

// hill
pub const HILL: Sprite = Sprite {
    width: 16,
    height: 13,
    flags: 1, // BLIT_2BPP
    data: &[
        0x00, 0x01, 0x40, 0x00, 0x00, 0x17, 0xd4, 0x00, 0x01, 0x7f, 0xfd, 0x40, 0x17, 0xff, 0xff,
        0xd4, 0x7f, 0xff, 0xff, 0xfd, 0x57, 0xff, 0xff, 0xd5, 0x69, 0x7f, 0xfd, 0x69, 0x6a, 0x97,
        0xd6, 0xa9, 0x6a, 0xa9, 0x6a, 0xa9, 0x16, 0xaa, 0xaa, 0x94, 0x01, 0x6a, 0xa9, 0x40, 0x00,
        0x16, 0x94, 0x00, 0x00, 0x01, 0x40, 0x00,
    ],
    colors: 0x3120,
};

// tower
pub const TOWER: Sprite = Sprite {
    width: 24,
    height: 32,
    flags: 1, // BLIT_2BPP
    data: &[
        0x00, 0x00, 0x55, 0x55, 0x00, 0x00, 0x00, 0x15, 0xff, 0xff, 0x54, 0x00, 0x01, 0x7f, 0xff,
        0xff, 0xfd, 0x40, 0x07, 0xff, 0xff, 0xff, 0xff, 0xd0, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xf4,
        0x1f, 0xff, 0xff, 0xff, 0xff, 0xf4, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xf4, 0x17, 0xff, 0xff,
        0xff, 0xff, 0xd4, 0x19, 0x7f, 0xff, 0xff, 0xfd, 0x64, 0x1a, 0x95, 0xff, 0xff, 0x56, 0xa4,
        0x1a, 0xaa, 0x55, 0x55, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa,
        0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4,
        0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa,
        0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4,
        0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa,
        0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4,
        0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa, 0xaa, 0xaa, 0xa4, 0x1a, 0xaa, 0xaa,
        0xaa, 0xaa, 0xa4, 0x06, 0xaa, 0xaa, 0xaa, 0xaa, 0x90, 0x01, 0x6a, 0xaa, 0xaa, 0xa9, 0x40,
        0x00, 0x15, 0xaa, 0xaa, 0x54, 0x00, 0x00, 0x00, 0x55, 0x55, 0x00, 0x00,
    ],
    colors: 0x3210,
};

// tile
pub const TILE: Sprite = Sprite {
    width: 16,
    height: 8,
    flags: 1, // BLIT_2BPP
    data: &[
        0x00, 0x16, 0x94, 0x00, 0x01, 0x6a, 0xa9, 0x40, 0x16, 0xaa, 0xaa, 0x94, 0x6a, 0xaa, 0xaa,
        0xa9, 0x16, 0xaa, 0xaa, 0x94, 0x01, 0x6a, 0xa9, 0x40, 0x00, 0x16, 0x94, 0x00, 0x00, 0x01,
        0x40, 0x00,
    ],
    colors: 0x310,
};

// portal
pub const PORTAL: Sprite = Sprite {
    width: 12,
    height: 15,
    flags: 1, // BLIT_2BPP
    data: &[
        0x00, 0x55, 0x00, 0x01, 0xaa, 0x40, 0x07, 0xff, 0x90, 0x1f, 0xab, 0xe4, 0x1e, 0xfe, 0xf4,
        0x7b, 0xef, 0xb9, 0x7b, 0xbb, 0xb9, 0x7b, 0xbb, 0xb9, 0x7b, 0xbf, 0xb9, 0x7b, 0xea, 0xf9,
        0x1e, 0xff, 0xe4, 0x1f, 0xaa, 0xb4, 0x07, 0xff, 0xd0, 0x01, 0xaa, 0x40, 0x00, 0x55, 0x00,
    ],
    colors: 0x4310,
};

// zombie
pub const ZOMBIE: Sprite = Sprite {
    width: 8,
    height: 7,
    flags: 0, // BLIT_1BPP
    data: &[0x30, 0x10, 0x60, 0xb0, 0x10, 0x38, 0x48],
    colors: 0x40,
};

// arrow_tower
pub const ARROW_TOWER: Sprite = Sprite {
    width: 8,
    height: 8,
    flags: 1, // BLIT_2BPP
    data: &[
        0x15, 0x45, 0x6a, 0x9d, 0x1d, 0x74, 0x05, 0xd9, 0x07, 0x59, 0x1d, 0x79, 0x04, 0x19, 0x00,
        0x04,
    ],
    colors: 0x4210,
};
