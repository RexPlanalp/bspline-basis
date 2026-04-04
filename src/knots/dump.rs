use crate::knots::knot_vector::KnotVector;
use crate::scalar::BSplineScalar;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;


pub fn dump_knots<T, K>(kv: &K) -> std::io::Result<()> 
where
    T: BSplineScalar,
    K: KnotVector<T>
    {
        let path = Path::new("output").join(kv.get_outfile());
        let output_file = File::create(path)?;
        let mut writer = BufWriter::new(output_file);

        for x in kv.get_knots() {
            writeln!(writer, "{} {}", x.re(), x.im())?;
        }

        Ok(())
    }