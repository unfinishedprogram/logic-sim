pub mod svg {
    pub const DOT: &str = include_str!("../assets/objects/gates/dot.svg");

    pub mod gates {
        pub const AND: &str = include_str!("../assets/objects/gates/and.svg");
        pub const OR: &str = include_str!("../assets/objects/gates/or.svg");
        pub const XOR: &str = include_str!("../assets/objects/gates/xor.svg");
        pub const NOT: &str = include_str!("../assets/objects/gates/not.svg");
        pub const BUF: &str = include_str!("../assets/objects/gates/buf.svg");
        pub const NAND: &str = include_str!("../assets/objects/gates/nand.svg");
        pub const NOR: &str = include_str!("../assets/objects/gates/nor.svg");
        pub const XNOR: &str = include_str!("../assets/objects/gates/xnor.svg");
    }
}

pub mod fonts {
    pub mod msdf {
        pub mod custom {
            pub const IMAGE: &[u8] = include_bytes!("../assets/custom.png");
            pub const MANIFEST: &str = include_str!("../assets/custom-msdf.json");
        }
    }
}
