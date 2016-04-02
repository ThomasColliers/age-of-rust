use math3d::matrix::Matrix4x4;
use num::{Float,NumCast,cast,Zero,One};
use std::f32;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Frustum<N: Float> {
	pub projection_matrix:Matrix4x4<N>
}

impl<N: Float+NumCast+Zero+One> Frustum<N> {
	pub fn new() -> Frustum<N> {
		Frustum {
			projection_matrix:Matrix4x4::<N>::identity()
		}
	}
	pub fn set_perspective(&mut self, fov:N, aspect_ratio:N, near:N, far:N){
        // calculate the near clipping plane
        // TODO: this number casting might not be ideal, maybe using traits on the numbers is more ideal
		let nymax = near * (fov * cast(f32::consts::PI).unwrap() / cast(360).unwrap()).tan();
        let nymin = -nymax;
        let nxmin = nymin * aspect_ratio;
        let nxmax = -nxmin;

        // update the projection matrix (creating new one right now)
        let zero = N::zero();
        let one = N::one();
        let two = one + one;
        self.projection_matrix = Matrix4x4::<N>::identity();
        self.projection_matrix.m11 = (two * near) / (nxmax - nxmin);
        self.projection_matrix.m22 = (two * near) / (nymax - nymin);
        self.projection_matrix.m31 = (nxmax + nxmin) / (nxmax - nxmin);
        self.projection_matrix.m32 = (nymax + nymin) / (nymax - nymin);
        self.projection_matrix.m33 = -((far + near) / (far - near));
        self.projection_matrix.m34 = -one;
        self.projection_matrix.m43 = -((two * far * near) / (far - near));
        self.projection_matrix.m44 = zero;
	}
	pub fn set_orthographic(&mut self){

	}
}

/*


void Frustum::setPerspective(GLfloat fov, GLfloat aspect, GLfloat near, GLfloat far){
    float nxmin, nxmax, nymin, nymax; // dimensions of near clipping plane;
    float fxmin, fxmax, fymin, fymax; // dimensions of far clipping plane;

    // near clipping plane
    nymax = near * float(tan(fov * PI / 360.0));
    nymin = -nymax;
    nxmin = nymin * aspect;
    nxmax = -nxmin;

    // construct projection matrix
    loadIdentity44(projectionMatrix);
    projectionMatrix[0] = (2.0f * near)/(nxmax - nxmin);
    projectionMatrix[5] = (2.0f * near)/(nymax - nymin);
    projectionMatrix[8] = (nxmax + nxmin) / (nxmax - nxmin);
    projectionMatrix[9] = (nymax + nymin) / (nymax - nymin);
    projectionMatrix[10] = -((far + near) / (far - near));
    projectionMatrix[11] = -1.0f;
    projectionMatrix[14] = -((2.0f * far * near)/(far - near));
    projectionMatrix[15] = 0.0f;

    // do the math for the far clipping plane
    fymax = far * float(tan(fov * PI / 360.0));
    fymin = -fymax;
    fxmin = fymin * aspect;
    fxmax = -fxmin;

    // values for transformed frustum corners
    nearUL[0] = nxmin; nearUL[1] = nymax; nearUL[2] = -near; nearUL[3] = 1.0f;
    nearLL[0] = nxmin; nearLL[1] = nymin; nearLL[2] = -near; nearLL[3] = 1.0f;
    nearUR[0] = nxmax; nearUR[1] = nymax; nearUR[2] = -near; nearUR[3] = 1.0f;
    nearLR[0] = nxmax; nearLR[1] = nymin; nearLR[2] = -near; nearLR[3] = 1.0f;
    farUL[0] = fxmin; farUL[1] = fymax; farUL[2] = -far; farUL[3] = 1.0f;
    farLL[0] = fxmin; farLL[1] = fymin; farLL[2] = -far; farLL[3] = 1.0f;
    farUR[0] = fxmax; farUR[1] = fymax; farUR[2] = -far; farUR[3] = 1.0f;
    farLR[0] = fxmax; farLR[1] = fymin; farLR[2] = -far; farLR[3] = 1.0f;
}

void Frustum::setOrthographic(GLfloat xmin, GLfloat xmax, GLfloat ymin, GLfloat ymax, GLfloat zmin, GLfloat zmax){
    makeOrthographicMatrix(projectionMatrix, xmin, xmax, ymin, ymax, zmin, zmax);
    projectionMatrix[15] = 1.0f;

    // values for untransformed frustum corners
    nearUL[0] = xmin; nearUL[1] = ymax; nearUL[2] = zmin; nearUL[3] = 1.0f;
    nearLL[0] = xmin; nearLL[1] = ymin; nearLL[2] = zmin; nearLL[3] = 1.0f;
    nearUR[0] = xmax; nearUR[1] = ymax; nearUR[2] = zmin; nearUR[3] = 1.0f;
    nearLR[0] = xmax; nearLR[1] = ymin; nearLR[2] = zmin; nearLR[3] = 1.0f;
    farUL[0] = xmin; farUL[1] = ymax; farUL[2] = zmax; farUL[3] = 1.0f;
    farLL[0] = xmin; farLL[1] = ymin; farLL[2] = zmax; farLL[3] = 1.0f;
    farUR[0] = xmax; farUR[1] = ymax; farUR[2] = zmax; farUR[3] = 1.0f;
    farLR[0] = xmax; farLR[1] = ymin; farLR[2] = zmax; farLR[3] = 1.0f;
}

*/