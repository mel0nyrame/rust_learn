use rust_learn::scan_modules;
fn main() {
    let modules = scan_modules();
    println!("当前 crate 共有 {} 个模块:", modules.len());
    for name in &modules {
        println!("  - {}", name);
    }
}