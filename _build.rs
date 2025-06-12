// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = "src/grpc_generated";
    std::fs::create_dir_all(out_dir).unwrap_or(());

    tonic_build::configure()
        .out_dir(out_dir)
        .compile(
            &["proto/axon_control.proto", "proto/echo.proto"], // Добавили echo.proto
            &["proto/"],
        )?;
    
    // Переименовываем оба сгенерированных файла
    let generated_control = std::path::Path::new(out_dir).join("axon.control.v1.rs");
    if generated_control.exists() {
        std::fs::rename(&generated_control, std::path::Path::new(out_dir).join("axon_control_v1.rs"))?;
    }

    let generated_echo = std::path::Path::new(out_dir).join("echo.v1.rs");
    if generated_echo.exists() {
        std::fs::rename(&generated_echo, std::path::Path::new(out_dir).join("echo_v1.rs"))?;
    }

    Ok(())
}