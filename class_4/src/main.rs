mod traffic;
mod math_own;
mod area;
use traffic::*;
use math_own::*;
use area::*;


fn main() {
    // 1
    println!("red: {}",TrafficLight::Red.time());
    println!("yellow: {}",TrafficLight::Yellow.time());
    println!("green: {}",TrafficLight::Green.time());



    println!("=================");
    println!("=================");


    // 2
    if let Some(s) = total(&[1u32,2u32,3u32,4u32]){
        println!("normal: {}",s);
    }
    if total(&[1u32,u32::MAX]).is_none(){
        println!("overflow");
    }


    println!("=================");
    println!("=================");


    // 3
    // 长方形 长3宽4
    let rectangle = Rect{width:3, height:4};
    println!("rectangle :{}", rectangle.area());

    // 圆形 半径 2，π 3.14
    let circular = Rect{width:3.14, height:2};
    println!("circular :{}", circular.area());

    // 正方形 宽3
    let square = Rect{width:3, height:3};
    println!("square :{}", square.area());

}
