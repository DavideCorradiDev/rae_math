use crate::{geometry2, geometry3, matrix::RealField};

pub trait ToHomogeneous3<N: RealField> {
    fn to_homogeneous3(&self) -> geometry3::HomogeneousMatrix<N>;
}

fn homogeneous2_to_homogeneous3<N: RealField>(
    m2: geometry2::HomogeneousMatrix<N>,
) -> geometry3::HomogeneousMatrix<N> {
    let mut out = geometry3::HomogeneousMatrix::<N>::identity();
    out[(0, 0)] = m2[(0, 0)];
    out[(0, 1)] = m2[(0, 1)];
    out[(0, 3)] = m2[(0, 2)];
    out[(1, 0)] = m2[(1, 0)];
    out[(1, 1)] = m2[(1, 1)];
    out[(1, 3)] = m2[(1, 2)];
    out
}

macro_rules! implement_to_homogeneous3 {
    ($StructType:ty) => {
        impl<N: RealField> ToHomogeneous3<N> for $StructType {
            fn to_homogeneous3(&self) -> geometry3::HomogeneousMatrix<N> {
                homogeneous2_to_homogeneous3(self.to_homogeneous())
            }
        }
    };
}

implement_to_homogeneous3!(geometry2::Affine<N>);
implement_to_homogeneous3!(geometry2::Isometry<N>);
implement_to_homogeneous3!(geometry2::Projective<N>);
implement_to_homogeneous3!(geometry2::Rotation<N>);
implement_to_homogeneous3!(geometry2::Similarity<N>);
implement_to_homogeneous3!(geometry2::Transform<N>);
implement_to_homogeneous3!(geometry2::Translation<N>);
implement_to_homogeneous3!(geometry2::OrthographicProjection<N>);