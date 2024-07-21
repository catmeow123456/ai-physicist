// import exp-data.rs in "./experiments" folder
use ai_physicist::experiments::expdata::ExpData;
use ndarray::{Array1, Array2};
use std::collections::HashMap;

// use symbolica::{atom::Atom, state::State};


fn test_symbolica() {
    // let input = Atom::parse("x(t)^2").unwrap();
    // let a = input.derivative(State::get_symbol("t"));
    // println!("d({})/dt = {}:", input, a);
}
fn test_expdata() {
    // let x: Array2<f64> = Array2::from_elem((5, 100), 1.0);
    // let xdata = ExpData::new(x);
    // let zero = ExpData::zero(5, 100);
    // let ans = &xdata / &zero;
    // // let st = s t;
    // for i in ans.badpts.iter() {
    //     print!("{} ", i);
    // }
    // println!("{}", xdata);
    // println!("{}", zero);
    // let ansclone = ans.clone();
    // println!("{}", ansclone);
    // println!("{}", ans);
}

use plotly::{Plot, Scatter};
use ai_physicist::experiments::expstructure::ExpStructure;
use ai_physicist::experiments::simulation::collision::Collision;

fn plot_expdata(data: &HashMap<String, ExpData>) {
    // plot the arr
    let mut plot = Plot::new();
    let t= data.get("t").unwrap().data.row(0).to_vec();
    for (key, value) in data.iter() {
        if key == "t" {
            continue;
        }
        let x = value.data.row(0).to_vec();
        let trace = Scatter::new(t.clone(), x.clone());
        plot.add_trace(trace);
    }
    // plot.show();
    plot.write_html("tmp/out.html");
}

fn test_experiments() {
    let mut exp: Collision = ExpStructure::new();
    // for (key, obj) in exp.obj_info().iter() {
    //     println!("NAME: {} , {}", key, obj);
    // }
    exp.random_sample();
    let data: HashMap<String, ExpData> = exp.get_expdata(10.0, 100, 1e-8, 10);
    plot_expdata(&data);
}
fn main() {
    // test_symbolica();
    // test_expdata();
    test_experiments();
}
