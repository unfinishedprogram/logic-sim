use crate::SVGAssets;

pub fn load_svg_assets<S, T>(source: SVGAssets<S>, load: fn(S) -> T) -> SVGAssets<T> {
    SVGAssets {
        dot_input: load(source.dot_input),
        dot_output: load(source.dot_output),

        gates: crate::GatesAssets {
            and_active: load(source.gates.and_active),
            and_normal: load(source.gates.and_normal),

            or_active: load(source.gates.or_active),
            or_normal: load(source.gates.or_normal),

            xor_active: load(source.gates.xor_active),
            xor_normal: load(source.gates.xor_normal),

            not_active: load(source.gates.not_active),
            not_normal: load(source.gates.not_normal),

            buf_active: load(source.gates.buf_active),
            buf_normal: load(source.gates.buf_normal),

            nand_active: load(source.gates.nand_active),
            nand_normal: load(source.gates.nand_normal),

            nor_active: load(source.gates.nor_active),
            nor_normal: load(source.gates.nor_normal),

            xnor_active: load(source.gates.xnor_active),
            xnor_normal: load(source.gates.xnor_normal),

            button_active: load(source.gates.button_active),
            button_normal: load(source.gates.button_normal),

            on_active: load(source.gates.on_active),
            on_normal: load(source.gates.on_normal),

            off_active: load(source.gates.off_active),
            off_normal: load(source.gates.off_normal),

            input: load(source.gates.input),
            output: load(source.gates.output),
        },

        ui: crate::UiAssets {
            button: load(source.ui.button),
            button_hover: load(source.ui.button_hover),
        },
    }
}
