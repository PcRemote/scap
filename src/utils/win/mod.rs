use windows_capture::graphics_capture_api::GraphicsCaptureApi;

pub fn is_supported() -> bool {
    GraphicsCaptureApi::is_supported().expect("Failed to check support") // TODO: > Windows 10 version 1903, May 2019 Update (19H1), build 10.0.18362.0
}
