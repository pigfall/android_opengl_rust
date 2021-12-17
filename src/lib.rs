use log::info;
use std::error::Error;
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

    match run(){
        Err(e) => info!("run over with error {:?}",e),
        _=> info!("run over"),
    }
}

fn run()->Result<(),Box<dyn Error>>{
    // < load egl TODO
    // 
    // >
    Ok(())

}
