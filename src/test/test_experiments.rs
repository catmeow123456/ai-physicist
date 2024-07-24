
#[cfg(test)]
mod tests {
    use crate::experiments::expstructure::ExpStructure;
    use crate::experiments::simulation::collision::Collision;
    use crate::experiments::simulation::oscillation::Oscillation;
    use std::time::Instant;
    use crate::sentence::{parse, eval};
    #[test]
    fn test_experiments() {
        {
            let now = Instant::now();
            let mut exp: Collision = ExpStructure::new();
            for (key, obj) in exp.obj_info().iter() {
                println!("NAME: {} , {}", key, obj);
            }
            println!();
            exp.random_sample();
            let data = exp.get_expdata(2.0, 100, 1e-8, 100);
            data.plot_expdata("collision");
            println!("Collision exp and plot, Time: {:?}", now.elapsed());
            let now = Instant::now();
            let expr = parse("(posx[1]'-posx[2]')**2").unwrap();
            let d = eval(&expr, &data);
            {
                assert!(d.is_conserved());
                assert!(! d.is_zero());
            }
            println!("Eval delta_v_square Time: {:?}", now.elapsed());
            let now = Instant::now();
            d.plot_over_t("delta_v_square", data.get_t());
            println!("Plot delta_v_square Time: {:?}", now.elapsed());
        }
        {
            let now = Instant::now();
            let mut exp: Oscillation = ExpStructure::new();
            exp.random_sample();
            let data = exp.get_expdata(2.0, 100, 1e-8, 100);
            data.plot_expdata("oscillation");
            println!("Oscillation Time: {:?}", now.elapsed());
            let expr = parse("D[posx[1]'']/D[ posx[1] ]").unwrap();
            let now = Instant::now();
            let d = eval(&expr, &data);
            {
                assert!(d.is_conserved());
                assert!(! d.is_zero());
                assert!(d.diff_tau().is_zero());
            }
            println!("Eval da/dt Time: {:?}", now.elapsed());
            let now = Instant::now();
            d.plot_over_t("da_dvd_dx", data.get_t());
            println!("Plot da/dt Time: {:?}", now.elapsed());
        }
    }
}