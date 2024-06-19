use nalgebra::{DMatrix,DVector};
fn ml_ij(n:&usize,i:usize,j:usize,length:&f64,masses:&Vec<f64>)-> f64 {
    let mut sum=0.0;
    for k in (std::cmp::max(i,j))..*n{
        sum+=masses[k];
    }
    sum*length
}

fn k_i(n:&usize,i:usize,masses:&Vec<f64>) -> f64 {
    let mut sum=0.0;
    for k in i..*n{
        sum+=masses[k]
    }
    sum
}
pub fn get_a_matrix(n:&usize, theta:&Vec<f64>,length:&Vec<f64>,mass:&Vec<f64>) -> DMatrix<f64> {
    DMatrix::from_fn(*n,*n,|i,j| ml_ij(n,i,j,&length[j],&mass)*(theta[i]-theta[j]).cos())
    
}

pub fn get_b_vector(n:&usize,theta:&Vec<f64>,theta_dot:&Vec<f64>,length:&Vec<f64>,mass:&Vec<f64>) -> DVector<f64>{
    
    DVector::from_fn(*n,|i,_| {let mut sum:f64=0.0;
        for j in 0..*n{
        sum+=(ml_ij(&n,i,j,&length[j],&mass))*theta_dot[j].powf(2.0)*(theta[i]-theta[j]).sin();
    }
    -sum-9.81*k_i(&n,i,&mass)*theta[i].sin() })
}

pub fn rk4_step(n:&usize,thetas:&Vec<f64>, theta_dots:&Vec<f64>,length:&Vec<f64>,masses:&Vec<f64>,h:f64) -> (Vec<f64>,Vec<f64>) {
    let k1 = {
        let mut theta_copy = thetas.clone();
        let mut theta_dot_copy=theta_dots.clone();
        theta_copy = theta_copy.iter().map(|x|x+0.0).collect::<Vec<f64>>();
        theta_dot_copy = theta_dot_copy.iter().map(|x|x+0.0).collect::<Vec<f64>>();
        let a=get_a_matrix(n, &theta_copy, &length, &masses);
        let b = get_b_vector(&n, &theta_copy, &theta_dot_copy, &length, &masses);
        let theta_dot:Vec<f64> = a.lu().solve(&b).unwrap().iter().map(|x|*x).collect::<Vec<f64>>();
        // println!("{:?}",theta_dot);
        let theta = theta_dot_copy;
        
        (theta,theta_dot)
    };
    let k2 = {
        let mut theta_copy = thetas.clone();
        let mut theta_dot_copy=theta_dots.clone();
        theta_copy = theta_copy.iter().zip(&k1.0).map(|(x,k)|x+h/2.0*k).collect::<Vec<f64>>();
        theta_dot_copy = theta_dot_copy.iter().zip(&k1.1).map(|(x,k)|x+h/2.0*k).collect::<Vec<f64>>();
        let a=get_a_matrix(n, &theta_copy, &length, &masses);
        let b = get_b_vector(&n, &theta_copy, &theta_dot_copy, &length, &masses);
        let theta_dot:Vec<f64> = a.lu().solve(&b).unwrap().iter().map(|x|*x).collect::<Vec<f64>>();
        

        let theta = theta_dot_copy;
        
        (theta,theta_dot)
    };
    let k3 = {
        let mut theta_copy = thetas.clone();
        let mut theta_dot_copy=theta_dots.clone();
        theta_copy = theta_copy.iter().zip(&k2.0).map(|(x,k)|x+h/2.0*k).collect::<Vec<f64>>();
        theta_dot_copy = theta_dot_copy.iter().zip(&k2.1).map(|(x,k)|x+h/2.0*k).collect::<Vec<f64>>();
        let a=get_a_matrix(n, &theta_copy, &length, &masses);
        let b = get_b_vector(&n, &theta_copy, &theta_dot_copy, &length, &masses);
        let theta_dot:Vec<f64> = a.lu().solve(&b).unwrap().iter().map(|x|*x).collect::<Vec<f64>>();
        
        let theta = theta_dot_copy;
        
        (theta,theta_dot)
    };
    let k4 = {
        let mut theta_copy = thetas.clone();
        let mut theta_dot_copy=theta_dots.clone();
        theta_copy = theta_copy.iter().zip(&k3.0).map(|(x,k)|x+h*k).collect::<Vec<f64>>();
        theta_dot_copy = theta_dot_copy.iter().zip(&k3.1).map(|(x,k)|x+h*k).collect::<Vec<f64>>();
        let a=get_a_matrix(n, &theta_copy, &length, &masses);
        let b = get_b_vector(&n, &theta_copy, &theta_dot_copy, &length, &masses);
        let theta_dot:Vec<f64> = a.lu().solve(&b).unwrap().iter().map(|x|*x).collect::<Vec<f64>>();
        // println!("{:?}",theta_dot);

        let theta = theta_dot_copy;
        
        (theta,theta_dot)
    };
    let mut theta = vec![0.0;*n];
    let mut theta_dot = vec![0.0;*n];
    for i in 0..*n{
        theta[i] = thetas[i]+h/6.0*(k1.0[i]+2.0*k2.0[i]+2.0*k3.0[i]+k4.0[i]);
        theta_dot[i] = theta_dots[i]+h/6.0*(k1.1[i]+2.0*k2.1[i]+2.0*k3.1[i]+k4.1[i]);
    }
    
    (theta,theta_dot)
}


// This is here in the case I want to re use this function for other code to generate and store the positions of moving objects

//TODO!
// pub fn rk4(n:usize,t0:f64,tf:f64,h:f64,init_theta:Vec<f64>,init_theta_dot:Vec<f64>,lengths:Vec<f64>,masses:Vec<f64>) -> (Vec<f64>,Vec<(Vec<f64>,Vec<f64>)>){
//     let m: usize = ((tf-t0)/h).floor() as usize;
//     let mut tout = (0..=m).into_iter().map(|t| t0+t as f64*h).collect::<Vec<f64>>();
//     let mut xout = vec![(vec![0.0;n],vec![0.0;n]);m+1];
//     xout[0].0 = init_theta; 
//     xout[0].1 = init_theta_dot; 
//     // let mut x:(Vec<f64>,Vec<f64>) = (xout[0].0,xout[0].1);
//     for i in 0..m{
//         (xout[i+1].0,xout[i+1].1) = rk4_step(&n,&xout[i].0,&xout[i].1,&lengths,&masses,h);
//     }
//     if *tout.last().unwrap()<tf {
//         let h = tf-tout[m];
//         let x = rk4_step(&n,&xout.last().unwrap().0,&xout.last().unwrap().1,&lengths,&masses,h);
//         tout.push(tf);
//         xout.push((x.0,x.1));
//         // xout.1.push(x.1);
//     }
//     (tout,xout)
// }