#![feature(impl_trait_in_assoc_type)]

mod knots;
mod bspline;
mod mesh;

#[cfg(test)]
mod tests {
    use crate::bspline::spline_basis::SplineBasis;
    use crate::bspline::spline_curve::SplineCurve;
    use crate::knots::knot_vec::KnotVec;
    use nalgebra::point;
    use plotters::backend::BitMapBackend;
    use plotters::chart::ChartBuilder;
    use plotters::prelude::{IntoDrawingArea, LineSeries, RED, WHITE};

    #[test]
    fn knots() {
        let Xi1 = KnotVec::from_sorted(vec![0.0, 0.0, 0.5, 1.0, 1.0]);
        let Xi2 = KnotVec::<f64>::open(6, 2);
        let (m, Z): (Vec<_>, Vec<_>) = Xi1.breaks_with_multiplicity().unzip();
        println!("Z: {:?}", Z);
        println!("m: {:?}", m);
        println!("{}", Xi1);
        println!("{}", Xi2);
    }

    #[test]
    fn splines() {
        let n = 4;
        let p = 2;
        let knots = KnotVec::<f64>::open(n, p);
        let splines = SplineBasis::new(knots.clone(), n, p);

        let t = 0.6;
        let idx = splines.find_span(t).unwrap();
        println!("{}", knots);
        println!("index {} in interval [{}, {})", idx, knots[idx], knots[idx+1]);
        
        println!("{:?}", splines.eval(t));
    }

    #[test]
    fn spline_curves() {
        let n = 5;
        let p = 2;
        let knots = KnotVec::<f64>::open(n, p);
        let splines = SplineBasis::new(knots.clone(), n, p);
        let curve = SplineCurve::new(
            vec![point![-1.0, 0.0], point![-0.5, 0.7], point![0.0, 0.0], point![0.5, -0.7], point![1.0, 0.0]],
            splines
        ).unwrap();

        println!("{:?}", curve);

        let N = 1000;
        let mesh = curve.mesh(N);
        let data = mesh.into_iter();

        let root_area = BitMapBackend::new("spline_curve.png", (800, 800))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&root_area)
            .build_cartesian_2d(-1.5..1.5, -1.5..1.5)
            .unwrap();

        ctx.configure_mesh().draw().unwrap();
        ctx.draw_series(LineSeries::new(data.map(|x| (x[0], x[1])), RED)).unwrap();
    }
}
