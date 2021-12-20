use log::info;
use std::error::Error;
use glow::HasContext;
use std::{thread,time};
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
    info!("Loading egl functions");
    let  egl_ins = unsafe {
        egl::DynamicInstance::<egl::EGL1_4>::load_required().map_err(|e|{info!("Failed to load egl functions {:?}",e);e})?
    };
    info!("Loaded egl functions");
    // 
    // << get default display and init it
    info!("Getting default display");
    let display = egl_ins.get_display(egl::DEFAULT_DISPLAY).ok_or_else(||{let msg="noget default display";info!("{:?}",msg);msg})?;
    info!("Getted default display");
    info!("Initing display");
    egl_ins.initialize(display).map_err(|e|{
        info!("Failed to init display {:?}",e);
        e
    })?;
    info!("Inited display");

    // >>
    // << choose config which matched the attributes we wanted
    info!("Choose config which matched the attributes we wanted");
    let attributes:Vec<egl::Int> = [egl::RED_SIZE, 8, egl::GREEN_SIZE, 8, egl::BLUE_SIZE, 8, egl::NONE].to_vec();
    let config :egl::Config = egl_ins.choose_first_config(display,&attributes).
        map_err(|e|{
            info!("Failed to choose first config {:?}",e);
            e
        })?.
        ok_or_else(||{let msg = "No matched config ";info!("{:?}",msg);msg})?;
    info!("Config choosed");
    // >>

    // << create_context
    info!("Creating context");
    let ctx = egl_ins.create_context(display,config,None,None).map_err(
            |e|{
                info!("Failed to create context {:?}",e);
                e
            }
        )?;
    info!("Created context");
    // >>

    // << create window surface
    let native_window_rw_lock =ndk_glue::native_window();
    let native_window = native_window_rw_lock.as_ref().
        ok_or_else(
                ||{
                    let msg = "Failed to get native window";
                    info!("{:?}",msg);
                    msg
                }
            )?;
    let native_window_ptr =native_window.ptr().as_ptr();

    info!("Creating window surface");
    let surface = unsafe {
egl_ins.create_window_surface(display,config,native_window_ptr as egl::NativeWindowType,None).
        map_err(
                |e|{
                    info!("Failed to create window surface {:?}",e);
                    e
                }
            )?
    }; 
    info!("Created window surface");
    // >>

    // <<  Attach an EGL rendering context to EGL surfaces.
    info!("Attach an EGL rendering context to EGL surfaces");
    egl_ins.make_current(display,Some(surface),Some(surface),Some(ctx)).
        map_err(
                |e|{
                    info!("Failed to attach egl rendering context to egl surfaces");
                    e
                }
            )?;
    info!("Attached an EGL rendering context to EGL surfaces");
    // >>
    // >
    //

    // < Loading all gl functions
    info!("Loading gl functions");
    let gl = unsafe {
        glow::Context::from_loader_function_with_version_parse(
        |version_str|{
            // TODO
            info!("gl version {:?}",version_str);
            Ok(
                 glow::Version {
                 major: 1,
                 minor: 0,
                 is_embedded: true,
                 revision: None,
                 vendor_info: "tzz".to_string(),
            }

                )
        }
            ,
        |name|{
                info!("Loading {:?}",name);
                egl_ins.get_proc_address(name).
                    map_or(std::ptr::null(),|ptr|{
                        info!("Loaded {:?} {:?}",name,ptr);
                        ptr as *const _
                    })
            }).map_err(
                    |e|{
                        info!("{:?}",e);
                        e
                    }
                )?
    };
    info!("Loaded gl functions");
    // >
    
    let duration = time::Duration::from_millis(500);
    loop {
        info!("Drawing");
        unsafe{
            gl.clear_color(0.1,0.2,0.3,1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            egl_ins.swap_buffers(display,surface);
        };
        info!("Drawed");
        thread::sleep(duration);
    };
    Ok(())

}
