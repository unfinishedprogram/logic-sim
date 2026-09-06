mod loader;

pub use loader::load_svg_assets;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SVGSource(pub String);

macro_rules! asset {
    ($name:ident: $path:literal, ( $($args:tt)* )) => {
        pub static $name: std::sync::LazyLock<crate::SVGSource> = std::sync::LazyLock::new(|| {
            crate::SVGSource(format!(
                include_str!($path),
                $($args)*
            ))
        });
    };
}

pub struct SVGAssets<T> {
    pub dot_input: T,
    pub dot_output: T,

    pub gates: GatesAssets<T>,
    pub ui: UiAssets<T>,
}

pub struct UiAssets<T> {
    pub button: T,
    pub button_hover: T,
}

pub struct GatesAssets<T> {
    pub and_active: T,
    pub and_normal: T,

    pub or_active: T,
    pub or_normal: T,

    pub xor_active: T,
    pub xor_normal: T,

    pub not_active: T,
    pub not_normal: T,

    pub buf_active: T,
    pub buf_normal: T,

    pub nand_active: T,
    pub nand_normal: T,

    pub nor_active: T,
    pub nor_normal: T,

    pub xnor_active: T,
    pub xnor_normal: T,

    pub button_active: T,
    pub button_normal: T,

    pub on_active: T,
    pub on_normal: T,

    pub off_active: T,
    pub off_normal: T,

    pub input: T,
    pub output: T,
}

pub fn get_svg_source_assets() -> SVGAssets<SVGSource> {
    use self::svg::gates::*;
    use self::svg::ui::*;
    use self::svg::*;

    SVGAssets {
        dot_input: DOT_INPUT.clone(),
        dot_output: DOT_OUTPUT.clone(),

        gates: GatesAssets {
            and_active: AND_ACTIVE.clone(),
            and_normal: AND_NORMAL.clone(),

            or_active: OR_ACTIVE.clone(),
            or_normal: OR_NORMAL.clone(),

            xor_active: XOR_ACTIVE.clone(),
            xor_normal: XOR_NORMAL.clone(),

            not_active: NOT_ACTIVE.clone(),
            not_normal: NOT_NORMAL.clone(),

            buf_active: BUF_ACTIVE.clone(),
            buf_normal: BUF_NORMAL.clone(),

            nand_active: NAND_ACTIVE.clone(),
            nand_normal: NAND_NORMAL.clone(),

            nor_active: NOR_ACTIVE.clone(),
            nor_normal: NOR_NORMAL.clone(),

            xnor_active: XNOR_ACTIVE.clone(),
            xnor_normal: XNOR_NORMAL.clone(),

            button_active: BUTTON_ACTIVE.clone(),
            button_normal: BUTTON_NORMAL.clone(),

            on_active: ON_ACTIVE.clone(),
            on_normal: ON_NORMAL.clone(),

            off_active: OFF_ACTIVE.clone(),
            off_normal: OFF_NORMAL.clone(),

            input: INPUT.clone(),
            output: OUTPUT.clone(),
        },

        ui: UiAssets {
            button: BUTTON.clone(),
            button_hover: BUTTON_HOVER.clone(),
        },
    }
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

        asset!(ON_ACTIVE: "objects/gates/on.svg", (stroke = "4"));
        asset!(ON_NORMAL: "objects/gates/on.svg", (stroke = "0"));

        asset!(OFF_ACTIVE: "objects/gates/off.svg", (stroke = "4"));
        asset!(OFF_NORMAL: "objects/gates/off.svg", (stroke = "0"));

        asset!(INPUT: "objects/gates/input.svg", ());
        asset!(OUTPUT: "objects/gates/output.svg", ());
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
