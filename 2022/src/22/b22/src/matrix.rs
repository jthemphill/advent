pub type Mat2 = [[i64; 2]; 2];
pub type Vec3 = [i64; 3];
pub type Mat3 = [[i64; 3]; 3];

/**
 * Rotation matrices.
 */
pub type Rot3 = Mat3;
pub const IDENTITY: Rot3 = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
pub const X_90: Rot3 = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
pub const Y_90: Rot3 = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
pub const Z_90: Rot3 = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];
pub const X_180: Rot3 = matmul(X_90, X_90);
pub const Y_180: Rot3 = matmul(Y_90, Y_90);
pub const Z_180: Rot3 = matmul(Z_90, Z_90);
pub const X_270: Rot3 = matmul(X_180, X_90);
pub const Y_270: Rot3 = matmul(Y_180, Y_90);
pub const Z_270: Rot3 = matmul(Z_180, Z_90);

pub const fn vecmul(a: Mat3, b: Vec3) -> Vec3 {
    [vecdot(a[0], b), vecdot(a[1], b), vecdot(a[2], b)]
}

pub const fn matmul(a: Mat3, b: Mat3) -> Mat3 {
    [
        [
            vecdot(a[0], col0(b)),
            vecdot(a[0], col1(b)),
            vecdot(a[0], col2(b)),
        ],
        [
            vecdot(a[1], col0(b)),
            vecdot(a[1], col1(b)),
            vecdot(a[1], col2(b)),
        ],
        [
            vecdot(a[2], col0(b)),
            vecdot(a[2], col1(b)),
            vecdot(a[2], col2(b)),
        ],
    ]
}

pub const fn transpose(a: Mat3) -> Mat3 {
    [
        [a[0][0], a[1][0], a[2][0]],
        [a[0][1], a[1][1], a[2][1]],
        [a[0][2], a[1][2], a[2][2]],
    ]
}

const fn scalar_vecdiv(a: Vec3, x: i64) -> Vec3 {
    [a[0] * x, a[1] * x, a[2] * x]
}

const fn scalar_matdiv(a: Mat3, x: i64) -> Mat3 {
    [
        scalar_vecdiv(a[0], x),
        scalar_vecdiv(a[1], x),
        scalar_vecdiv(a[2], x),
    ]
}

const fn det2(a: Mat2) -> i64 {
    a[0][0] * a[1][1] - a[0][1] * a[1][0]
}

const fn det3(a: Mat3) -> i64 {
    a[0][0] * a[1][1] * a[2][2] + a[0][1] * a[1][2] * a[2][0] + a[0][2] * a[1][0] * a[2][1]
        - a[0][0] * a[1][2] * a[2][1]
        - a[0][1] * a[1][0] * a[2][2]
        - a[0][2] * a[1][1] * a[2][0]
}

const fn vecdot(a: Vec3, b: Vec3) -> i64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

const fn col0(a: Mat3) -> Vec3 {
    [a[0][0], a[1][0], a[2][0]]
}

const fn col1(a: Mat3) -> Vec3 {
    [a[0][1], a[1][1], a[2][1]]
}

const fn col2(a: Mat3) -> Vec3 {
    [a[0][2], a[1][2], a[2][2]]
}

const fn matmul_verbose(a: Mat3, b: Mat3) -> Mat3 {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
        ],
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
        ],
    ]
}

fn matmul_old(a: Mat3, b: Mat3) -> Mat3 {
    let mut ret = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                ret[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    ret
}

const fn inverse(a: Mat3) -> Mat3 {
    scalar_matdiv(transpose(cofactor(a)), det3(a))
}

const fn cofactor(a: Mat3) -> Mat3 {
    [
        [
            det2([[a[1][1], a[1][2]], [a[2][1], a[2][2]]]),
            -det2([[a[1][0], a[1][2]], [a[2][0], a[2][2]]]),
            det2([[a[1][0], a[1][1]], [a[2][0], a[2][1]]]),
        ],
        [
            -det2([[a[0][1], a[0][2]], [a[2][1], a[2][2]]]),
            det2([[a[0][0], a[0][2]], [a[2][0], a[2][2]]]),
            -det2([[a[0][0], a[0][1]], [a[2][0], a[2][1]]]),
        ],
        [
            det2([[a[0][1], a[0][2]], [a[1][1], a[1][2]]]),
            -det2([[a[0][0], a[0][2]], [a[1][0], a[1][2]]]),
            det2([[a[0][0], a[0][1]], [a[1][0], a[1][1]]]),
        ],
    ]
}

#[test]
fn test_axis_rotations() {
    assert_eq!(vecmul(X_90, [1, 0, 0]), [1, 0, 0]);
    assert_eq!(vecmul(X_90, [0, 1, 0]), [0, 0, 1]);
    assert_eq!(vecmul(X_90, [0, 0, 1]), [0, -1, 0]);

    assert_eq!(vecmul(Y_90, [1, 0, 0]), [0, 0, -1]);
    assert_eq!(vecmul(Y_90, [0, 1, 0]), [0, 1, 0]);
    assert_eq!(vecmul(Y_90, [0, 0, 1]), [1, 0, 0]);

    assert_eq!(vecmul(Z_90, [1, 0, 0]), [0, 1, 0]);
    assert_eq!(vecmul(Z_90, [0, 1, 0]), [-1, 0, 0]);
    assert_eq!(vecmul(Z_90, [0, 0, 1]), [0, 0, 1]);
}

#[test]
fn test_associative() {
    assert_eq!(vecmul(Z_90, [0, 1, 0]), [-1, 0, 0]);
    assert_eq!(vecmul(Y_90, [-1, 0, 0]), [0, 0, 1]);
    assert_eq!(vecmul(Y_90, vecmul(Z_90, [0, 1, 0])), [0, 0, 1]);
    assert_eq!(matmul(Y_90, Z_90), [[0, 0, 1], [1, 0, 0], [0, 1, 0]]);
    assert_eq!(vecmul(matmul(Y_90, Z_90), [0, 1, 0]), [0, 0, 1]);
}

#[test]
fn test_zyz() {
    // z90 * y90 * z270 = x90, right??

    assert_eq!(vecmul(X_90, [0, 1, 0]), [0, 0, 1]);

    assert_eq!(vecmul(Z_90, [0, 1, 0]), [-1, 0, 0]);
    assert_eq!(vecmul(Y_90, [-1, 0, 0]), [0, 0, 1]);
    assert_eq!(vecmul(Z_270, [0, 0, 1]), [0, 0, 1]);
    assert_eq!(
        vecmul(Z_270, vecmul(Y_90, vecmul(Z_90, [0, 1, 0]))),
        [0, 0, 1]
    );

    assert_eq!(
        vecmul(matmul(Z_270, matmul(Y_90, Z_90)), [0, 1, 0]),
        [0, 0, 1]
    );
    assert_eq!(matmul(Z_270, matmul(Y_90, Z_90)), X_90);
}

#[test]
fn test_transpose() {
    assert_eq!(transpose(X_90), X_270);
    assert_eq!(transpose(Y_90), Y_270);
    assert_eq!(transpose(Z_90), Z_270);
    assert_eq!(transpose(X_180), X_180);
    assert_eq!(transpose(Y_180), Y_180);
    assert_eq!(transpose(Z_180), Z_180);
}

#[test]
fn test_inverse() {
    assert_eq!(inverse(X_90), X_270);
    assert_eq!(inverse(Y_90), Y_270);
    assert_eq!(inverse(Z_90), Z_270);
    assert_eq!(inverse(X_180), X_180);
    assert_eq!(inverse(Y_180), Y_180);
    assert_eq!(inverse(Z_180), Z_180);
}

#[test]
fn const_matmul() {
    assert_eq!(matmul(X_90, Y_90), matmul_old(X_90, Y_90));
    assert_eq!(matmul_verbose(X_90, Y_90), matmul_old(X_90, Y_90));
}

#[test]
fn matmul_identity() {
    assert_eq!(matmul(IDENTITY, IDENTITY), IDENTITY);

    assert_eq!(matmul(IDENTITY, X_90), X_90);
    assert_eq!(matmul(X_90, IDENTITY), X_90);

    let mut a = IDENTITY;
    for _ in 0..4 {
        a = matmul(a, X_90);
    }
    assert_eq!(a, IDENTITY);
}

#[test]
fn test_z_rotation() {
    assert_eq!(matmul(X_90, Y_90), matmul(matmul(X_180, Y_90), Z_270));
    assert_eq!(matmul(X_270, Y_270), matmul(matmul(X_180, Y_270), Z_270));
}
