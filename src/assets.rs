macro_rules! asset {
    ($name:ident: $path:literal, ( $($args:tt)* )) => {
        pub static $name: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
            format!(
                include_str!($path),
                $($args)*
            )
        });
    };
}

pub mod svg {
    // Connection Dot
    asset!(DOT_INPUT: "../assets/objects/dot.svg", (color = "red", radius = "2"));
    asset!(DOT_OUTPUT: "../assets/objects/dot.svg", (color = "green", radius = "2"));

    pub mod gates {
        asset!(AND_ACTIVE: "../assets/objects/gates/and.svg", (stroke = "4"));
        asset!(AND_NORMAL: "../assets/objects/gates/and.svg", (stroke = "0"));

        asset!(OR_ACTIVE: "../assets/objects/gates/or.svg", (stroke = "4"));
        asset!(OR_NORMAL: "../assets/objects/gates/or.svg", (stroke = "0"));

        asset!(XOR_ACTIVE: "../assets/objects/gates/xor.svg", (stroke = "4"));
        asset!(XOR_NORMAL: "../assets/objects/gates/xor.svg", (stroke = "0"));

        asset!(NOT_ACTIVE: "../assets/objects/gates/not.svg", (stroke = "4"));
        asset!(NOT_NORMAL: "../assets/objects/gates/not.svg", (stroke = "0"));

        asset!(BUF_ACTIVE: "../assets/objects/gates/buf.svg", (stroke = "4"));
        asset!(BUF_NORMAL: "../assets/objects/gates/buf.svg", (stroke = "0"));

        asset!(NAND_ACTIVE: "../assets/objects/gates/nand.svg", (stroke = "4"));
        asset!(NAND_NORMAL: "../assets/objects/gates/nand.svg", (stroke = "0"));

        asset!(NOR_ACTIVE: "../assets/objects/gates/nor.svg", (stroke = "4"));
        asset!(NOR_NORMAL: "../assets/objects/gates/nor.svg", (stroke = "0"));

        asset!(XNOR_ACTIVE: "../assets/objects/gates/xnor.svg", (stroke = "4"));
        asset!(XNOR_NORMAL: "../assets/objects/gates/xnor.svg", (stroke = "0"));
    }

    pub mod ui {
        asset!(BUTTON: "../assets/objects/button_outline.svg", ());
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
