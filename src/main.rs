//! # cellinator
//!
//! `cellinator` is a very simple cell simulator. Hopefully it will become more powerfull overtime.
//!
//! ## Roadmap
//!
//! - [x] Simple diffusion implementation between equal volume solvents via permeating membranes
//! - [] Implement Fick's law based diffusion

/// Represents a solvent which in this case is always water
struct Solvent {
    o2_count: i128,
}

/// Represents a biological membrane with different permeabilities for different molecules
struct Membrane {
    o2_permeability: f32,
}

/// Objects may permeate substances between two solvents, by varying rate
trait Permeate {
    fn permeate(&self, solvent1: &mut Solvent, solvent2: &mut Solvent);
}

impl Permeate for Membrane {
    fn permeate(&self, solvent1: &mut Solvent, solvent2: &mut Solvent) {
        let o2_forward = (solvent1.o2_count as f32) * self.o2_permeability;
        let o2_reverse = (solvent2.o2_count as f32) * self.o2_permeability;

        solvent1.o2_count = solvent1.o2_count - (o2_forward.round() as i128);
        solvent2.o2_count = solvent2.o2_count - (o2_reverse.round() as i128);

        solvent1.o2_count = solvent1.o2_count + (o2_reverse.round() as i128);
        solvent2.o2_count = solvent2.o2_count + (o2_forward.round() as i128);
    }
}

fn main() {
    let mut extrasol = Solvent { o2_count: 10000 };

    let mut periplasm = Solvent { o2_count: 10 };

    let mut cytosol = Solvent { o2_count: 5 };

    let outer_membrane = Membrane {
        o2_permeability: 0.3,
    };

    let inner_membrane = Membrane {
        o2_permeability: 0.3,
    };

    println!("cycle,extrasol_o2,periplasm_o2,cytosol_o2");
    for cycle in 1..=10 {
        println!(
            "{},{},{},{}",
            cycle, extrasol.o2_count, periplasm.o2_count, cytosol.o2_count
        );
        outer_membrane.permeate(&mut extrasol, &mut periplasm);
        inner_membrane.permeate(&mut periplasm, &mut cytosol);
    }
}
