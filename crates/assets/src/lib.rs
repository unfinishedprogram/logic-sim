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
    asset!(DOT_INPUT: "objects/dot.svg", (color = "red", radius = "2"));
    asset!(DOT_OUTPUT: "objects/dot.svg", (color = "green", radius = "2"));

    pub mod gates {
        asset!(AND_ACTIVE: "objects/gates/and.svg", (stroke = "4"));
        asset!(AND_NORMAL: "objects/gates/and.svg", (stroke = "0"));

        asset!(OR_ACTIVE: "objects/gates/or.svg", (stroke = "4"));
        asset!(OR_NORMAL: "objects/gates/or.svg", (stroke = "0"));

        asset!(XOR_ACTIVE: "objects/gates/xor.svg", (stroke = "4"));
        asset!(XOR_NORMAL: "objects/gates/xor.svg", (stroke = "0"));

        asset!(NOT_ACTIVE: "objects/gates/not.svg", (stroke = "4"));
        asset!(NOT_NORMAL: "objects/gates/not.svg", (stroke = "0"));

        asset!(BUF_ACTIVE: "objects/gates/buf.svg", (stroke = "4"));
        asset!(BUF_NORMAL: "objects/gates/buf.svg", (stroke = "0"));

        asset!(NAND_ACTIVE: "objects/gates/nand.svg", (stroke = "4"));
        asset!(NAND_NORMAL: "objects/gates/nand.svg", (stroke = "0"));

        asset!(NOR_ACTIVE: "objects/gates/nor.svg", (stroke = "4"));
        asset!(NOR_NORMAL: "objects/gates/nor.svg", (stroke = "0"));

        asset!(XNOR_ACTIVE: "objects/gates/xnor.svg", (stroke = "4"));
        asset!(XNOR_NORMAL: "objects/gates/xnor.svg", (stroke = "0"));

        asset!(BUTTON_ACTIVE: "objects/gates/button.svg", (stroke = "4"));
        asset!(BUTTON_NORMAL: "objects/gates/button.svg", (stroke = "0"));
    }

    pub mod ui {
        asset!(BUTTON: "objects/button_outline.svg", ());
        asset!(BUTTON_HOVER: "objects/button_outline_hover.svg", ());
    }
}

pub mod fonts {
    pub mod msdf {
        pub mod custom {
            pub const IMAGE: &[u8] = include_bytes!("font/custom.png");
            pub const MANIFEST: &str = include_str!("font/custom-msdf.json");
        }
    }
}
