use image_webp::WebPDecoder;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::BufReader;
use std::os::raw::c_char;

/// WebP图片信息结构体
#[derive(Debug)]
pub struct WebpInfo {
    pub width: u32,
    pub height: u32,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub num_frames: u32,
}

impl WebpInfo {
    fn new_valid(decoder: &WebPDecoder<BufReader<File>>) -> Self {
        WebpInfo {
            width: decoder.dimensions().0,
            height: decoder.dimensions().1,
            has_alpha: decoder.has_alpha(),
            is_animated: decoder.is_animated(),
            num_frames: decoder.num_frames(),
        }
    }
}

/// 校验WebP图片格式是否合法
pub fn validate_webp(path: &str) -> Result<WebpInfo, String> {
    let file = File::open(path).map_err(|e| format!("failed to open file: {}", e))?;
    let reader = BufReader::new(file);

    match WebPDecoder::new(reader) {
        Ok(decoder) => Ok(WebpInfo::new_valid(&decoder)),
        Err(e) => Err(format!("webp format validation failed: {:?}", e)),
    }
}

/// C兼容的WebP校验结果结构体
#[repr(C)]
pub struct WebpValidationResult {
    pub is_valid: bool,
    pub width: u32,
    pub height: u32,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub num_frames: u32,
    pub error_message: *mut c_char,
}

/// 通过FFI校验WebP文件
///
/// # Safety
/// 调用方必须确保：
/// 1. path是有效的以null结尾的C字符串
/// 2. 使用free_error_message释放error_message
#[no_mangle]
pub unsafe extern "C" fn validate_webp_ffi(path: *const c_char) -> WebpValidationResult {
    if path.is_null() {
        return WebpValidationResult {
            is_valid: false,
            width: 0,
            height: 0,
            has_alpha: false,
            is_animated: false,
            num_frames: 0,
            error_message: CString::new("path is null").unwrap().into_raw(),
        };
    }

    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return WebpValidationResult {
                is_valid: false,
                width: 0,
                height: 0,
                has_alpha: false,
                is_animated: false,
                num_frames: 0,
                error_message: CString::new("invalid utf-8 in path").unwrap().into_raw(),
            };
        }
    };

    match validate_webp(path_str) {
        Ok(info) => WebpValidationResult {
            is_valid: true,
            width: info.width,
            height: info.height,
            has_alpha: info.has_alpha,
            is_animated: info.is_animated,
            num_frames: info.num_frames,
            error_message: std::ptr::null_mut(),
        },
        Err(err) => WebpValidationResult {
            is_valid: false,
            width: 0,
            height: 0,
            has_alpha: false,
            is_animated: false,
            num_frames: 0,
            error_message: CString::new(err).unwrap().into_raw(),
        },
    }
}

/// 释放validate_webp_ffi分配的错误消息内存
///
/// # Safety
/// 调用方必须确保：
/// 1. error_message是由validate_webp_ffi返回的指针
/// 2. 每个指针只调用此函数一次
#[no_mangle]
pub unsafe extern "C" fn free_error_message(error_message: *mut c_char) {
    if !error_message.is_null() {
        unsafe {
            let _ = CString::from_raw(error_message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dynamic_webp() {
        let result = validate_webp("images/dynamic.webp");

        assert!(result.is_ok(), "dynamic webp should pass validation");

        let info = result.unwrap();
        assert!(info.is_animated, "should be identified as animated");
        assert!(
            info.num_frames > 1,
            "animated image should have multiple frames, actual: {}",
            info.num_frames
        );
        assert!(
            info.width > 0 && info.height > 0,
            "should have valid dimensions"
        );

        println!("dynamic webp test passed:");
        println!("  dimensions: {}x{}", info.width, info.height);
        println!("  has alpha: {}", info.has_alpha);
        println!("  frames: {}", info.num_frames);
    }

    #[test]
    fn test_validate_static_webp() {
        let result = validate_webp("images/static.webp");

        assert!(result.is_ok(), "static webp should pass validation");

        let info = result.unwrap();
        assert!(!info.is_animated, "should be identified as static");
        assert_eq!(info.num_frames, 0, "static image should have 0 frames");
        assert!(
            info.width > 0 && info.height > 0,
            "should have valid dimensions"
        );

        println!("static webp test passed:");
        println!("  dimensions: {}x{}", info.width, info.height);
        println!("  has alpha: {}", info.has_alpha);
    }

    #[test]
    fn test_validate_fake_webp() {
        let result = validate_webp("images/fake.webp");

        assert!(result.is_err(), "fake webp should fail validation");

        let error = result.unwrap_err();
        assert!(
            error.contains("webp format validation failed"),
            "error should contain 'webp format validation failed'"
        );
        assert!(
            error.contains("ChunkHeaderInvalid"),
            "error should contain 'ChunkHeaderInvalid', actual: {}",
            error
        );

        println!("fake webp test passed:");
        println!("  error message: {}", error);
    }

    #[test]
    fn test_validate_nonexistent_file() {
        let result = validate_webp("images/nonexistent.webp");

        assert!(result.is_err(), "nonexistent file should return error");

        let error = result.unwrap_err();
        assert!(
            error.contains("failed to open file"),
            "error should contain 'failed to open file'"
        );

        println!("nonexistent file test passed:");
        println!("  error message: {}", error);
    }

    #[test]
    fn test_webp_info_debug() {
        let result = validate_webp("images/static.webp");
        assert!(result.is_ok());

        let info = result.unwrap();
        let debug_str = format!("{:?}", info);

        assert!(
            debug_str.contains("WebpInfo"),
            "debug output should contain struct name"
        );
        println!("debug formatting test passed:");
        println!("  {:?}", info);
    }
}
