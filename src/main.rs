use ceres_solver::{CostFunctionType, NllsProblem, SolverOptions};

fn main() {
    // cut an pasted the example from the ceres-solver documentation
    
    // parameters vector consists of vector parameters, here we have a single 1-D parameter.
    let true_parameters = vec![vec![2.0]];
    let initial_parameters = vec![vec![0.0]];

    // You can skip type annotations in the closure definition, we use them for verbosity only.
    let cost: CostFunctionType = Box::new(
        move |parameters: &[&[f64]],
            residuals: &mut [f64],
            mut jacobians: Option<&mut [Option<&mut [&mut [f64]]>]>| {
            // residuals have the size of your data set, in our case it is just 1
            residuals[0] = parameters[0][0] - 2.0;
            // jacobians can be None, then you don't need to provide them
            if let Some(jacobians) = jacobians {
                // The size of the jacobians array is equal to the number of parameters,
                // each element is Option<&mut [&mut [f64]]>
                if let Some(d_dx) = &mut jacobians[0] {
                    // Each element in the jacobians array is slice of slices:
                    // the first index is for different residuals components,
                    // the second index is for different components of the parameter vector
                    d_dx[0][0] = 1.0;
                }
            }
            true
        },
    );
    
    let solution = NllsProblem::new()
        .residual_block_builder() // create a builder for residual block
        .set_cost(cost, 1) // 1 is the number of residuals
        .set_parameters(initial_parameters)
        .build_into_problem()
        .unwrap()
        .0 // build_into_problem returns a tuple (NllsProblem, ResidualBlockId)
        // You can repeat .residual_block_builder() and .build_into_problem() calls to add more
        // residual blocks
        .solve(&SolverOptions::default()) // SolverOptions can be customized
        .unwrap(); // Err should happen only if we added no residual blocks

    // Print the full solver report
    println!("{}", solution.summary.full_report());

    // The solution is a vector of parameter vectors, here we have a single 1-D parameter.
    assert!(f64::abs(solution.parameters[0][0] - true_parameters[0][0]) < 1e-8);
}
