use ai_physicist::experiments::expstructure::ExpStructure;
use ai_physicist::experiments::simulation::collision::Collision;
use ai_physicist::experiments::simulation::oscillation::Oscillation;
use ndarray::array;
use std::time::Instant;


use ai_physicist::sentence::{parse, eval};
fn test_experiments() {
    let mut exp: Collision = ExpStructure::new();
    // for (key, obj) in exp.obj_info().iter() {
    //     println!("NAME: {} , {}", key, obj);
    // }
    exp.random_sample();
    let data = exp.get_expdata(2.0, 100, 1e-8, 100);
    data.plot_expdata("collision");
    let expr = parse("(posx[1]'-posx[2]')**2").unwrap();
    let d = eval(&expr, &data);
    d.plot_over_t("delta_v_square", data.get_t());

    let now = Instant::now();
    let mut exp: Oscillation = ExpStructure::new();
    exp.random_sample();
    let data = exp.get_expdata(2.0, 100, 1e-8, 100);
    // data.plot_expdata("oscillation");
    // let expr = parse("posx[1]'").unwrap();
    let expr = parse("D[posx[1]'']/D[ posx[1] ]").unwrap();
    println!("{}", expr);
    println!("Time: {:?}", now.elapsed());
    let now = Instant::now();
    let d = eval(&expr, &data);
    println!("Time: {:?}", now.elapsed());
    let now = Instant::now();
    d.plot_over_t("da_dvd_dx", data.get_t());
    println!("Time: {:?}", now.elapsed());
}

use ai_physicist::experiments::expdata::NPSCoefficient;
fn main() {
    println!("{:?}", NPSCoefficient::new(5).c);
    // let x = ExpData::new(array![[1.0, 2.0, 4.0, 10.0, 13.0, 16.0, 17.0]]);
    // let x = x.diff_tau();
    // print!("{:?}", x);

    // let x = parse("(x + y*(z+x)**2)**2").unwrap();
    // let x = parse("D[x]/D[t]").unwrap();
    // println!("{}", x);
    test_experiments();
}
