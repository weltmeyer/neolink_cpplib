pub use neolink_core::{
    //bc_protocol::{StreamOutput, StreamOutputError},
    bcmedia::model::*,
};

use neolink_core::bc_protocol::BcCamera;
use neolink_core::bc_protocol::StreamKind;
use neolink_core::bc_protocol::BcCameraOpt;
use neolink_core::bc_protocol::ConnectionProtocol;
use neolink_core::bc_protocol::Credentials;
use neolink_core::bc_protocol::DiscoveryMethods;
use std::collections::HashMap;
//use neolink_core::bc_protocol::{self, Stream};
use lazy_static::lazy_static;
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::thread;
use std::{
   // fmt::{Display, Error as FmtError, Formatter},
    net::{IpAddr/*, ToSocketAddrs*/},
    str::FromStr,
};
use std::net::SocketAddr;
use tokio::runtime::Runtime;
//pub use neolink_core::bc_protocol::Error;

#[repr(C)]
pub enum FrameType {
    /// H264 video data
    H264 = 0,
    /// H265 video data
    H265 = 1,
    AAC = 2,
    AdPCM = 3,
}

pub struct ExtOutputs {
    //frametype
    //seconds since 1970
    //data pointer
    //data length
    //microseconds
    pub frame_func: unsafe extern "C" fn(FrameType, u32, *mut u8, i32, u32),
    pub info_func: unsafe extern "C" fn(u32, u32, u8), //widh,height,fps
}

lazy_static! {
    static ref RT: Runtime = Runtime::new().unwrap();
    static ref LOG_INIT: bool = false;
   
}
/*
lazy_static! {
    static ref CAMS: HashMap<u64,BcCamera>=HashMap::new();
    static ref CAMNUMBER:u64 = 1;
   
}*/

//
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[no_mangle]
pub extern "C" fn lib_test() {
    println!("Hello from the library!");
}

///opens a camera connection
#[no_mangle]
pub extern "C" fn lib_cam_open(
    c_ipaddress: *const c_char,
    c_username: *const c_char,
    c_password: *const c_char,
) -> *mut BcCamera {

        env_logger::try_init();
    
    let ipaddress = string_from_c(c_ipaddress);
    let password = string_from_c(c_password);
    let username = string_from_c(c_username);
    println!("Hello from the library, host:{}!", ipaddress);

    let socketaddr: SocketAddr = ipaddress.parse().unwrap();
    //let ipadr=IpAddr::from_str(&ipaddress).unwrap();
    let ipadr=socketaddr.ip();
    let finalAddr=vec![ipadr];
   let name="Extern";
    let options = BcCameraOpt {
        name: name.to_string(),
        channel_id: 0,
        addrs: finalAddr,
        port: Some(socketaddr.port()),
        uid: None,
        protocol: ConnectionProtocol::Tcp,
        discovery: DiscoveryMethods::None,
        credentials: Credentials {
            username: username,
            password: Some(password),
        },
        debug: false,
        max_discovery_retries: 0,
    };

    //let mut rt = Runtime::new().unwrap();
    let mut camera: BcCamera = RT.block_on(async { BcCamera::new(&options).await}).unwrap();
    /*RT.block_on(async  {camera
        .login().await});*/
    

    return Box::into_raw(Box::new(camera));
}

///starts camera stream main
#[no_mangle]
pub extern "C" fn lib_cam_start_stream(
    ptr: *const BcCamera,
    newdata: unsafe extern "C" fn(FrameType, u32, *mut u8, i32, u32),
    info: unsafe extern "C" fn(u32, u32, u8), //width,height,fps
) {
    let mut ext_output: ExtOutputs = ExtOutputs {
        frame_func: newdata,
        info_func: info,
    };

    let cam:&BcCamera = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    //thread::spawn(move || {
		
		//let mut rt = Runtime::new().unwrap();
		//let block_on = RT.block_on(
            RT.spawn(
            async move{
                println!("hello from the async block");
                let loginResult=cam.login().await.expect("Bad Login data");
                println!("IAMLOGGEDIN");
                unsafe { (ext_output.info_func)(loginResult.resolution.width, loginResult.resolution.height, 0) };
                let mut stream_data=cam.start_video(StreamKind::Main,09999,true).await.expect("JW:error1");
                //let mut stream_data = camera.start_video(name, 0, strict).await?;

                

                loop {
                    log::debug!("Waiting for frame");
                    
                    let data = match stream_data.get_data().await{
                        Ok(x)=>x.expect("JW:error2"),
                        Err(e)=>break
                    };
                    

                   
                    let mut frame_type = FrameType::H264;
                    let mut timestamp = 0;
                    let mut payloaddata: Vec<u8> = Vec::new();
                    let mut microseconds: u32 = 0;
                    //let data1=data.unwrap();
                    //let data2=data1.unwrap();
                    log::debug!("Nice1:a1");
                    match &data {
                        BcMedia::InfoV1(payload) => {
                            log::debug!("---Info1---");
                            unsafe { (ext_output.info_func)(payload.video_width, payload.video_height, payload.fps) };
                        },
                        BcMedia::InfoV2(payload) => {
                            log::debug!("---Info2---");
                            unsafe { (ext_output.info_func)(payload.video_width, payload.video_height, payload.fps) };
                        },

                        _ => {
                            //println!("{}", std::any::type_name::<T>())
                           
                            //print_type_of(&data);
                            log::debug!("XXX:unk1:XXX");
                        }
                    }
                    match data{
                        BcMedia::Iframe(payload) => {
                            frame_type = match payload.video_type {
                                VideoType::H264 => FrameType::H264,
                                VideoType::H265 => FrameType::H265,
                            };
                            microseconds = payload.microseconds;
                            payloaddata = payload.data;
                            timestamp = payload.time.unwrap_or(0);
                        },
                        BcMedia::Pframe(payload) => {
                            frame_type = match payload.video_type {
                                VideoType::H264 => FrameType::H264,
                                VideoType::H265 => FrameType::H265,
                            };
                            microseconds = payload.microseconds;
                            payloaddata = payload.data;
                        },
                        BcMedia::Aac(payload) => {
                            payloaddata = payload.data;
                            //microseconds = payload.microseconds;
                            frame_type = FrameType::AAC;
                        },
                        BcMedia::Adpcm(payload) => {
                            payloaddata = payload.data;
                            //microseconds = payload.microseconds;
                            frame_type = FrameType::AdPCM;
                        },
                        BcMedia::InfoV1(payload) => {
                            log::debug!("---Info1---");
                            unsafe { (ext_output.info_func)(payload.video_width, payload.video_height, payload.fps) };
                        },
                        BcMedia::InfoV2(payload) => {
                            log::debug!("---Info2---");
                            unsafe { (ext_output.info_func)(payload.video_width, payload.video_height, payload.fps) };
                        },

                        _ => {
                            log::debug!("XXX:UNK2:XXX");
                        }
                    }
                    log::debug!("Nice1:a2");
                    if payloaddata.len() > 0 {
                        let data_length = payloaddata.len().try_into().unwrap();
                        let data_ptr = payloaddata.as_mut_ptr();
                        unsafe {
                            (ext_output.frame_func)(frame_type, timestamp, data_ptr, data_length, microseconds);
                        }
                    }
                    log::debug!("Nice1:a3");
                    
                }


            //bonus, you could spawn tasks too
            //tokio::spawn(async { async_function("task1").await });
            //tokio::spawn(async { async_function("task2").await });
            
        });
        /*cam.start_video(&mut ext_output, Stream::Main)
            .map_err(|e| println!("error:{}!", e))
            .ok();*/

        println!("Run finished.");
    //});
}

#[no_mangle]
pub extern "C" fn lib_cam_stop(ptr: *mut BcCamera) {
    let cam = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    println!("Shutdown...");

    //let mut rt = Runtime::new().unwrap();
    RT.block_on(
        async {
            cam.stop_video(StreamKind::Main).await;
            cam.shutdown().await;
        }
    );


    println!("Shutdown!");
    println!("Join..");
    let cam:&BcCamera = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    
    RT.block_on(
        async {
            cam.join().await;
        }
    );
    println!("Join!");
}

pub fn string_from_c(s: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    return r_str.to_string();
}
