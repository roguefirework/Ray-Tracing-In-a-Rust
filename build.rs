use vcpkg;
fn main() {
    vcpkg::find_package("ffmpeg").unwrap();
}