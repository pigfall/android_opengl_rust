use log::info;
#[cfg_attr(
    target_os="android",
    ndk_glue::main(
        backtrace="on",
        logger(
            tag = "opengl_demo",
            level="debug",
            ),
        )
    )]
pub fn main(){
    info!("calling main");

}
