use art::kinds::PrimaryColor;
use art::utils::mix;

// 对项目结构重新导出之后，可以直接导入，而忽略其中的结构
// use art::PrimaryColor;
// use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}