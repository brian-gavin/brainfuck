./target/release/brainfuck -o "$1.ssa" "$1.bf" && qbe -o "$1.s" "$1.ssa" && cc "$1.s"
