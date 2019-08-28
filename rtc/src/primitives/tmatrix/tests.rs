#![cfg(test)]

use super::*;
use crate::{matrix, utils::typelevel_nums::*};

#[test]
fn iter() {
    let a = matrix![ N2, N3 =>
        1 2 3;
        5 6 7
    ];
    let mut b = a.iter();
    assert_eq!(b.next(), Some(&1));
    assert_eq!(b.next(), Some(&2));
    assert_eq!(b.next(), Some(&3));
    assert_eq!(b.next(), Some(&5));
    assert_eq!(b.next(), Some(&6));
    assert_eq!(b.next(), Some(&7));
    assert_eq!(b.next(), None);
}

#[test]
fn iter_col() {
    let a = matrix![ N4, N3 =>
        1 2 3;
        5 6 7;
        9 8 7;
        6 5 4
    ];
    assert_eq!(
        a.iter_col(0).map(|x| *x).collect::<Vec<_>>(),
        vec![1, 5, 9, 6]
    );
    assert_eq!(
        a.iter_col(1).map(|x| *x).collect::<Vec<_>>(),
        vec![2, 6, 8, 5]
    );
    assert_eq!(
        a.iter_col(2).map(|x| *x).collect::<Vec<_>>(),
        vec![3, 7, 7, 4]
    );
}

#[test]
fn iter_row() {
    let a = matrix![ N3, N4 =>
        1 5 9 6;
        2 6 8 5;
        3 7 7 4
    ];
    assert_eq!(
        a.iter_row(0).map(|x| *x).collect::<Vec<_>>(),
        vec![1, 5, 9, 6]
    );
    assert_eq!(
        a.iter_row(1).map(|x| *x).collect::<Vec<_>>(),
        vec![2, 6, 8, 5]
    );
    assert_eq!(
        a.iter_row(2).map(|x| *x).collect::<Vec<_>>(),
        vec![3, 7, 7, 4]
    );
}

#[test]
fn iter_rows() {
    let a = matrix![ N4, N3 =>
        1 2 3;
        5 6 7;
        9 8 7;
        6 5 4
    ];
    let mut b = a.iter_rows();
    assert_eq!(
        b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert_eq!(
        b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
        vec![5, 6, 7]
    );
    assert_eq!(
        b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
        vec![9, 8, 7]
    );
    assert_eq!(
        b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
        vec![6, 5, 4]
    );
}

#[test]
fn iter_cols() {
    let a = matrix![ N4, N3 =>
        1 2 3;
        5 6 7;
        9 8 7;
        6 5 4
    ];
    let mut b = a.iter_cols();
    assert_eq!(
        b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
        vec![1, 5, 9, 6]
    );
    assert_eq!(
        b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
        vec![2, 6, 8, 5]
    );
    assert_eq!(
        b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
        vec![3, 7, 7, 4]
    );
}

#[test]
fn index() {
    let a = matrix![ N6, N3 =>
        1, 2, 3;
        5, 6, 7;
        9, 8, 7;
        6, 5, 4;
        3, 2, 1;
        0, -1, -2
    ];
    assert_eq!(a[(2, 0)], 9);
}

#[test]
fn mul() {
    let a = matrix![ N3, N4 =>
        1 2 3 4;
        5 6 7 8;
        9 8 7 6
    ];
    let b = matrix![ N4, N4 =>
        -2, 1, 2, 3;
        3, 2, 1, -1;
        4, 3, 6, 5;
        1, 2, 7, 8
    ];
    let c = matrix![ N3, N4 =>
        20 22 50 48;
        44 54 114 108;
        40 58 110 102
    ];
    assert_eq!(a * b, c);
    let a = matrix![ N4, N4 =>
        1 2 3 4;
        5 6 7 8;
        9 8 7 6;
        5 4 3 2
    ];
    let b = matrix![ N4, N1 =>
        2;
        3;
        4;
        1
    ];
    let c = matrix![ N4, N1 => 24; 64; 76; 36];
    assert_eq!(a * b, c);
}

#[test]
fn identity() {
    let a = matrix![ N3, N3 =>
        1 2 3;
        4 5 6;
        7 8 9
    ];
    assert_eq!(a.clone() * <Matrix<_, N3, N3>>::identity(), a.clone());
}

#[test]
fn det() {
    let a = matrix![ N2, N2 =>
        1 5 ;
        {-3} 2
    ];
    assert_eq!(a.det(), 17);

    let b = matrix![ N3, N3 =>
        1, 2, 6;
        -5, 8, -4;
        2, 6, 4
    ];
    assert_eq!(b.cofactor(0, 0), 56);
    assert_eq!(b.cofactor(0, 1), 12);
    assert_eq!(b.cofactor(0, 2), -46);
    assert_eq!(b.det(), -196);

    let c = matrix![ N4, N4 =>
        -2, -8, 3, 5;
        -3, 1, 7, 3;
        1, 2, -9, 6;
        -6, 7, 7, -9
    ];
    assert_eq!(c.cofactor(0, 0), 690);
    assert_eq!(c.cofactor(0, 1), 447);
    assert_eq!(c.cofactor(0, 2), 210);
    assert_eq!(c.cofactor(0, 3), 51);
    assert_eq!(c.det(), -4071);
}

#[test]
fn remove_row() {
    let a = matrix![ N5, N5 =>
        1 5 3 4 5;
        2 2 7 9 8;
        3 2 1 7 9;
        5 5 3 6 1;
        0 5 0 2 4
    ];
    assert_eq!(
        a.remove_row(0),
        matrix![ N4, N5 =>
            2 2 7 9 8;
            3 2 1 7 9;
            5 5 3 6 1;
            0 5 0 2 4
        ]
    );
}

#[test]
fn remove_col() {
    let a = matrix![ N5, N5 =>
        1 5 3 4 5;
        2 2 7 9 8;
        3 2 1 7 9;
        5 5 3 6 1;
        0 5 0 2 4
    ];
    assert_eq!(
        a.remove_col(2),
        matrix![ N5, N4 =>
            1 5 4 5;
            2 2 9 8;
            3 2 7 9;
            5 5 6 1;
            0 5 2 4
        ]
    );
}

#[test]
fn submatrix() {
    let a = matrix![ N3, N3 =>
        1, 5, 0;
        -3, 2, 2;
        0, 6, -3
    ];
    assert_eq!(
        a.submatrix(0, 2),
        matrix![ N2, N2 =>
            -3, 2;
            0, 6
        ]
    );
    let b = matrix![ N4, N4 =>
        -6, 1, 1, 6;
        -8, 5, 8, 6;
        -1, 0, 8, 2;
        -7, 1, -1, 1
    ];
    assert_eq!(
        b.submatrix(2, 1),
        matrix![ N3, N3 =>
            -6, 1, 6;
            -8, 8, 6;
            -7, -1, 1
        ]
    );
}

#[test]
fn minor() {
    let a = matrix![ N3, N3 =>
        3, 5, 0;
        2, -1, -7;
        6, -1, 5
    ];
    let b = a.submatrix(1, 0);
    assert_eq!(a.minor(1, 0), 25);
    assert_eq!(a.minor(1, 0), b.det());
}

#[test]
fn invert() {
    let a = matrix![ N4, N4 =>
        6, 4, 4, 4;
        5, 5, 7, 6;
        4, -9, 3, -7;
        9, 1, 7, -6
    ];
    assert_eq!(a.det(), -2120);
    assert!(a.invert().is_some());

    let b = matrix![ N4, N4 =>
        -4.,  2., -2., -3.;
            9.,  6.,  2.,  6.;
            0., -5.,  1., -5.;
            0.,  0.,  0.,  0.
    ];
    assert_eq!(b.det(), 0.0);
    assert!(b.invert().is_none());

    let c = matrix![ N4, N4 =>
        -5.,  2.,  6., -8.;
            1., -5.,  1.,  8.;
            7.,  7., -6., -7.;
            1., -3.,  7.,  4.
    ];
    let c_inv = matrix![ N4, N4 =>
            0.21805,  0.45113,  0.24060, -0.04511;
        -0.80827, -1.45677, -0.44361,  0.52068;
        -0.07895, -0.22368, -0.05263,  0.19737;
        -0.52256, -0.81391, -0.30075,  0.30639
    ];
    assert!(c.det().approx_eq(532.0));
    assert!(c.cofactor(2, 3).approx_eq(-160.0));
    assert!(c_inv[(3, 2)].approx_eq(-160.0 / 532.0));
    assert!(c.cofactor(3, 2).approx_eq(105.0));
    assert!(c_inv[(2, 3)].approx_eq(105.0 / 532.0));
    assert!(c.invert().unwrap().approx_eq(&c_inv));

    let d = matrix![ N4, N4 =>
            8., -5.,  9.,  2.;
            7.,  5.,  6.,  1.;
        -6.,  0.,  9.,  6.;
        -3.,  0., -9., -4.
    ];
    assert!(d.invert().unwrap().approx_eq(&matrix![ N4, N4 =>
        -0.15385, -0.15385, -0.28205, -0.53846;
        -0.07692,  0.12308,  0.02564,  0.03077;
            0.35897,  0.35897,  0.43590,  0.92308;
        -0.69231, -0.69231, -0.76923, -1.92308
    ]));

    let e = matrix![ N4, N4 =>
            9.,  3.,  0.,  9.;
        -5., -2., -6., -3.;
        -4.,  9.,  6.,  4.;
        -7.,  6.,  6.,  2.
    ];
    assert!(e.invert().unwrap().approx_eq(&matrix![ N4, N4 =>
        -0.04074, -0.07778,  0.14444, -0.22222;
        -0.07778,  0.03333,  0.36667, -0.33333;
        -0.02901, -0.14630, -0.10926,  0.12963;
            0.17778,  0.06667, -0.26667,  0.33333
    ]));
}

#[test]
fn inverse_multiplication() {
    let a = matrix![ N4, N4 =>
            3., -9.,  7.,  3.;
            3., -8.,  2., -9.;
        -4.,  4.,  4.,  1.;
        -6.,  5., -1.,  1.
    ];
    let b = matrix![ N4, N4 =>
        8.,  2., 2., 2.;
        3., -1., 7., 0.;
        7.,  0., 5., 4.;
        6., -2., 0., 5.
    ];
    assert!((a.clone() * b.clone() * b.invert().unwrap()).approx_eq(&a));
}

#[test]
fn add() {
    let a = matrix![ N3, N2 =>
        1 3;
        6 2;
        9 0
    ];
    let b = matrix![ N3, N2 =>
        0 0;
        4 2;
        7 8
    ];
    assert_eq!(
        a + b,
        matrix![ N3, N2 =>
            1 3;
            10 4;
            16 8
        ]
    );
}

#[test]
fn add_assign() {
    let mut a = matrix![ N3, N2 =>
        1 3;
        6 2;
        9 0
    ];
    let b = matrix![ N3, N2 =>
        0 0;
        4 2;
        7 8
    ];
    a += b;
    assert_eq!(
        a,
        matrix![ N3, N2 =>
            1 3;
            10 4;
            16 8
        ]
    );
}

#[test]
fn sub() {
    let a = matrix![ N3, N2 =>
        1 3;
        6 2;
        9 0
    ];
    let b = matrix![ N3, N2 =>
        0 0;
        4 2;
        7 8
    ];
    assert_eq!(
        a - b,
        matrix![ N3, N2 =>
                1,  3;
                2,  0;
                2, -8
        ]
    );
}

#[test]
fn neg() {
    let a = matrix![ N3, N2 =>
        1 3;
        6 2;
        9 0
    ];
    assert_eq!(
        -a,
        matrix![ N3, N2 =>
            -1, -3;
            -6, -2;
            -9,  0
        ]
    );
}

#[test]
fn mul_scalar() {
    let a = matrix![ N3, N2 =>
        1 3;
        6 2;
        9 0
    ];
    assert_eq!(
        a * 5,
        matrix![ N3, N2 =>
            5,  15;
            30, 10;
            45,  0
        ]
    );
}

#[test]
fn div_scalar() {
    let a = matrix![ N3, N2 =>
        15 3;
        6 18;
        9 0
    ];
    assert_eq!(
        a / 3,
        matrix![ N3, N2 =>
            5,  1;
            2,  6;
            3,  0
        ]
    );
}
