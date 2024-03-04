use crate::matrix::{ MatrixBuilder, MatrixDefinitionError };
use crate::square::Square;
use crate::symmetric::{ Symmetric, Algorithm };
use crate::definite::{ CholeskyDecompositionError, PositiveDefinite };


#[test]
fn test_builder() {
    let data = vec![0, 1, 2, 3];
    assert!(matches!(MatrixBuilder::new().rows(2).cols(2).data(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().rows(4).cols(1).data(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().rows(1).cols(4).data(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().rows(0).cols(4).data(data.clone()).build(), Err(MatrixDefinitionError::UndefinedRows)));
    assert!(matches!(MatrixBuilder::new().rows(4).cols(0).data(data.clone()).build(), Err(MatrixDefinitionError::UndefinedCols)));
    assert!(matches!(MatrixBuilder::new().rows(2).cols(5).data(data.clone()).build(), Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)));

    let data: Vec<i32> = vec![];
    assert!(matches!(MatrixBuilder::new().rows(2).cols(2).data(data.clone()).build(), Err(MatrixDefinitionError::NoDataProvided)));

    let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    assert!(matches!(MatrixBuilder::new().rows(3).cols(5).data(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().rows(5).cols(3).data(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().rows(15).cols(1).data(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().rows(0).cols(1).data( data.clone()).build(), Err(MatrixDefinitionError::UndefinedRows)));
    assert!(matches!(MatrixBuilder::new().rows(15).cols(0).data(data.clone()).build(), Err(MatrixDefinitionError::UndefinedCols)));
    assert!(matches!(MatrixBuilder::new().rows(4).cols(4).data(data.clone()).build(), Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)));
}

#[test]
fn test_builder_from_vec() {
    let data = vec![0, 1, 2, 3];
    assert!(matches!(MatrixBuilder::new().from_vec(2, 2, data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_vec(4, 1, data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_vec(1, 4, data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_vec(0, 4, data.clone()).build(), Err(MatrixDefinitionError::UndefinedRows)));
    assert!(matches!(MatrixBuilder::new().from_vec(4, 0, data.clone()).build(), Err(MatrixDefinitionError::UndefinedCols)));
    assert!(matches!(MatrixBuilder::new().from_vec(2, 5, data.clone()).build(), Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)));

    let data: Vec<i32> = vec![];
    assert!(matches!(MatrixBuilder::new().from_vec(2, 2, data.clone()).build(), Err(MatrixDefinitionError::NoDataProvided)));

    let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    assert!(matches!(MatrixBuilder::new().from_vec(3, 5, data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_vec(5, 3, data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_vec(15, 1, data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_vec(0, 15, data.clone()).build(), Err(MatrixDefinitionError::UndefinedRows)));
    assert!(matches!(MatrixBuilder::new().from_vec(15, 0, data.clone()).build(), Err(MatrixDefinitionError::UndefinedCols)));
    assert!(matches!(MatrixBuilder::new().from_vec(4, 4, data.clone()).build(), Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)));
}

#[test]
fn test_builder_from_mat() {
    let data = vec![
        vec![0, 1], 
        vec![2, 3],
    ];
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Ok(_)));
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Ok(_)));

    let data = vec![
        vec![0, 1], 
        vec![2, 3, 4],
        vec![5, 6, 7],
    ];
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)));

    let data = vec![
        vec![0, 1, 2], 
        vec![3, 4],
        vec![5, 6, 7],
    ];
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)));

    let data = vec![
        vec![0, 1, 2], 
        vec![3, 4, 5],
        vec![6, 7],
    ];
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)));

    let data: Vec<Vec<i32>> = vec![];
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Err(MatrixDefinitionError::NoDataProvided)));

    let data: Vec<Vec<i32>> = vec![vec![]];
    assert!(matches!(MatrixBuilder::new().from_mat(data.clone()).build(), Err(MatrixDefinitionError::NoDataProvided)));
}

#[test]
fn test_builder_zeros() {
    assert_eq!(
        MatrixBuilder::<f64>::new().zeros(3, 3).build().unwrap(),
        MatrixBuilder::<f64>::new().from_mat(
            vec![
                vec![0f64, 0f64, 0f64],
                vec![0f64, 0f64, 0f64], 
                vec![0f64, 0f64, 0f64],
            ]
        ).build().unwrap()
    );

    assert_eq!(
        MatrixBuilder::<i32>::new().zeros(5, 3).build().unwrap(),
        MatrixBuilder::<i32>::new().from_mat(
            vec![
                vec![0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32], 
                vec![0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32],
            ]
        ).build().unwrap()
    );

    assert_eq!(
        MatrixBuilder::<i64>::new().zeros(3, 5).build().unwrap(),
        MatrixBuilder::<i64>::new().from_mat(
            vec![
                vec![0i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64],
            ]
        ).build().unwrap()
    );

}

#[test]
fn test_builder_identity() {
    assert_eq!(
        MatrixBuilder::<f32>::new().identity(3).build().unwrap(),
        MatrixBuilder::<f32>::new().from_mat(
            vec![
                vec![1f32, 0f32, 0f32],
                vec![0f32, 1f32, 0f32], 
                vec![0f32, 0f32, 1f32],
            ]
        ).build().unwrap()
    );

    assert_eq!(
        MatrixBuilder::<f64>::new().identity(3).build().unwrap(),
        MatrixBuilder::<f64>::new().from_mat(
            vec![
                vec![1f64, 0f64, 0f64],
                vec![0f64, 1f64, 0f64], 
                vec![0f64, 0f64, 1f64],
            ]
        ).build().unwrap()
    );
    assert_eq!(
        MatrixBuilder::<i32>::new().identity(11).build().unwrap(),
        MatrixBuilder::<i32>::new().from_mat(
            vec![
                vec![1i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
                vec![0i32, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32], 
                vec![0i32, 0i32, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32, 0i32, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 0i32, 0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 0i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 0i32, 0i32],
                vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 0i32],
                vec![0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32],
            ]
        ).build().unwrap()
    );

    assert_eq!(
        MatrixBuilder::<i64>::new().identity(11).build().unwrap(),
        MatrixBuilder::<i64>::new().from_mat(
            vec![
                vec![1i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 1i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64], 
                vec![0i64, 0i64, 1i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 1i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 1i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64, 1i64, 0i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 1i64, 0i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 1i64, 0i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 1i64, 0i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 1i64, 0i64],
                vec![0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 1i64],
            ]
        ).build().unwrap()
    );
}

#[test]
fn test_is_square() {
    let data = vec![
        vec![0, 1, 2, 3, 4], 
        vec![5, 6, 7, 8, 9],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_square());
    let data = vec![
        vec![0, 1],
        vec![2, 3],
        vec![4, 5],
        vec![6, 7],
        vec![8, 9], 
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_square());
    let data = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(matrix.is_square());
}

#[test]
fn test_is_symmetric() {
    let data = vec![
        vec![0, 1], 
        vec![1, 3],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(matrix.is_symmetric());
    let data = vec![
        vec![4, 1, 2], 
        vec![1, 5, 3],
        vec![2, 3, 6],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(matrix.is_symmetric());
    let data = vec![
        vec![4, 1, 2, 0], 
        vec![1, 5, 3, 0],
        vec![2, 3, 6, 0],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_symmetric());
    let data = vec![
        vec![4, 1, 2], 
        vec![1, 5, 3],
        vec![2, 3, 6],
        vec![0, 0, 0],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_symmetric());
    let data = vec![
        vec![4i64, 1i64 / 3i64, 2i64 / 3i64], 
        vec![1i64 / 3i64, 5, 3i64],
        vec![2i64 / 3i64, 3, 6i64],
        vec![0i64, 0i64, 0i64],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_symmetric());
    let data = vec![
        vec![4i64, 1i64 / 11i64, 2i64 / 11i64], 
        vec![1i64 / 11i64, 5, 3i64],
        vec![2i64 / 11i64, 3, 6i64],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(matrix.is_symmetric());
}

#[test]
fn test_jacobi() {
    // (1, 1, 1)-axis 45 deg rotation matrix
    let m = MatrixBuilder::new().from_mat(
        vec![
            vec![1f32, 4f32, 5f32],
            vec![4f32, 2f32, 6f32],
            vec![5f32, 6f32, 3f32],
        ]
    ).build().unwrap();
    let (d, p) = match m.eigen_decomposition(Algorithm::Jacobi) {
        Ok(result) => result,
        Err(_) => panic!("Jacobi Algorithm went wrong"),
    };
    assert_eq!(d, vec![12.175971065046905, -2.507287967093641, -3.6686830979532656]);
    assert_eq!(
        p,
        MatrixBuilder::new().from_mat(
            vec![
                vec![ 0.49659978454619136, 0.8095854617397509, -0.31298567719355896],
                vec![0.5773502691896256, -0.5773502691896253, -0.5773502691896264], 
                vec![0.6481167492476514, -0.10600965430705542, 0.7541264035547062],
            ]
        ).build().unwrap()
    );

    let m = MatrixBuilder::new().from_mat(
        vec![
            vec![1f64, 4f64, 5f64],
            vec![4f64, 2f64, 6f64],
            vec![5f64, 6f64, 3f64],
        ]
    ).build().unwrap();
    let (d, p) = match m.eigen_decomposition(Algorithm::Jacobi) {
        Ok(result) => result,
        Err(_) => panic!("Jacobi Algorithm went wrong"),
    };
    assert_eq!(d, vec![12.175971065046905, -2.507287967093641, -3.6686830979532656]);
    assert_eq!(
        p,
        MatrixBuilder::new().from_mat(
            vec![
                vec![ 0.49659978454619136, 0.8095854617397509, -0.31298567719355896],
                vec![0.5773502691896256, -0.5773502691896253, -0.5773502691896264], 
                vec![0.6481167492476514, -0.10600965430705542, 0.7541264035547062],
            ]
        ).build().unwrap()
    );

    let m = MatrixBuilder::new().from_mat(
        vec![
            vec![1i32, 4i32, 5i32],
            vec![4i32, 2i32, 6i32],
            vec![5i32, 6i32, 3i32],
        ]
    ).build().unwrap();
    let (d, p) = match m.eigen_decomposition(Algorithm::Jacobi) {
        Ok(result) => result,
        Err(_) => panic!("Jacobi Algorithm went wrong"),
    };
    assert_eq!(d, vec![12.175971065046905, -2.507287967093641, -3.6686830979532656]);
    assert_eq!(
        p,
        MatrixBuilder::new().from_mat(
            vec![
                vec![ 0.49659978454619136, 0.8095854617397509, -0.31298567719355896],
                vec![0.5773502691896256, -0.5773502691896253, -0.5773502691896264], 
                vec![0.6481167492476514, -0.10600965430705542, 0.7541264035547062],
            ]
        ).build().unwrap()
    );

    let m = MatrixBuilder::new().from_mat(
        vec![
            vec![1i64, 4i64, 5i64],
            vec![4i64, 2i64, 6i64],
            vec![5i64, 6i64, 3i64],
        ]
    ).build().unwrap();
    let (d, p) = match m.eigen_decomposition(Algorithm::Jacobi) {
        Ok(result) => result,
        Err(_) => panic!("Jacobi Algorithm went wrong"),
    };
    assert_eq!(d, vec![12.175971065046905, -2.507287967093641, -3.6686830979532656]);
    assert_eq!(
        p,
        MatrixBuilder::new().from_mat(
            vec![
                vec![ 0.49659978454619136, 0.8095854617397509, -0.31298567719355896],
                vec![0.5773502691896256, -0.5773502691896253, -0.5773502691896264], 
                vec![0.6481167492476514, -0.10600965430705542, 0.7541264035547062],
            ]
        ).build().unwrap()
    );

    let m = MatrixBuilder::new().from_mat(
        vec![
            vec![1f32, 4f32, 5f32, 7f32],
            vec![4f32, 2f32, 6f32, 8f32],
            vec![5f32, 6f32, 3f32, 9f32],
            vec![7f32, 8f32, 9f32, 10f32],
        ]
    ).build().unwrap();
    let (d, p) = match m.eigen_decomposition(Algorithm::Jacobi) {
        Ok(result) => result,
        Err(_) => panic!("Jacobi Algorithm went wrong"),
    };
    assert_eq!(d, vec![25.058622745690624, -2.495082466558941, -2.8944982219584907, -3.669042057173195]);
    assert_eq!(
        p,
        MatrixBuilder::new().from_mat(
            vec![
                vec![0.3666086337260453, 0.7376057579081058, -0.4675015271226198, -0.3209021310561052],
                vec![0.42281150217481533, -0.63684003110579, -0.271973514038792, -0.5845473599899699],
                vec![0.4751461302778105, -0.1846796992232606, -0.4301817263823432, 0.7450323790705575],
                vec![0.6790174505770971, 0.12753795269170323, 0.7227837934584009, 0.01590472724922168],
            ]
        ).build().unwrap()
    );
}

#[test]
fn test_is_positive_definite() {
    let data = vec![
        vec![0, 1], 
        vec![1, 3],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_positive_definite());
    let data = vec![
        vec![4, 1], 
        vec![1, 3],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(matrix.is_positive_definite());

    let data = vec![
        vec![4, 1, 2], 
        vec![1, 5, 3],
        vec![2, 3, 6],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(matrix.is_positive_definite());
    let data = vec![
        vec![-4, 1, 2], 
        vec![1, 5, 3],
        vec![2, 3, 6],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_positive_definite());
    let data = vec![
        vec![4, 1, 2, 0], 
        vec![1, 5, 3, 0],
        vec![2, 3, 6, 0],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_positive_definite());
    let data = vec![
        vec![4, 1, 2], 
        vec![1, 5, 3],
        vec![2, 3, 6],
        vec![0, 0, 0],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_positive_definite());
    let data = vec![
        vec![4i64, 1i64 / 3i64, 2i64 / 3i64], 
        vec![1i64 / 3i64, 5i64, 3i64],
        vec![2i64 / 3i64, 3i64, 6i64],
        vec![0i64, 0i64, 0i64],
    ];

    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(!matrix.is_positive_definite());
    let data = vec![
        vec![4f64, 1f64 / 11f64, 2f64 / 11f64], 
        vec![1f64 / 11f64, 5f64, 3f64],
        vec![2f64 / 11f64, 3f64, 6f64],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    assert!(matrix.is_positive_definite());
}

#[test]
fn test_cholesky() {
    let data = vec![
        vec![0, 1], 
        vec![1, 3],
    ];
    let matrix = MatrixBuilder::new().from_mat(data).build().unwrap();
    println!("{:?}", matrix.cholesky());
    assert_eq!(matrix.cholesky(), Err(CholeskyDecompositionError::MatrixIsNotPositiveDefinite));
    let a = MatrixBuilder::new().from_mat(
        vec![
            vec![  1i32,   2i32,   4i32,   7i32],
            vec![  2i32,  13i32,  23i32,  38i32],
            vec![  4i32,  23i32,  77i32, 122i32],
            vec![  7i32,  38i32, 122i32, 294i32]
        ]
    ).build().unwrap();
    let l = MatrixBuilder::new().from_mat(
        vec![
            vec![1f64, 0f64, 0f64, 0f64], 
            vec![2f64, 3f64, 0f64, 0f64], 
            vec![4f64, 5f64, 6f64, 0f64], 
            vec![7f64, 8f64, 9f64, 10f64]
        ]
    ).build().unwrap();
    assert_eq!(l, a.cholesky().unwrap());
    let a = MatrixBuilder::new().from_mat(
        vec![
            vec![  1i64,   2i64,   4i64,   7i64],
            vec![  2i64,  13i64,  23i64,  38i64],
            vec![  4i64,  23i64,  77i64, 122i64],
            vec![  7i64,  38i64, 122i64, 294i64]
        ]
    ).build().unwrap();
    let l = MatrixBuilder::new().from_mat(
        vec![
            vec![1f64, 0f64, 0f64, 0f64], 
            vec![2f64, 3f64, 0f64, 0f64], 
            vec![4f64, 5f64, 6f64, 0f64], 
            vec![7f64, 8f64, 9f64, 10f64]
        ]
    ).build().unwrap();
    assert_eq!(l, a.cholesky().unwrap());
    let a = MatrixBuilder::new().from_mat(
        vec![
            vec![1.9383451f32 , 0.76780201f32, 1.4940289f32 , 0.75654844f32],
            vec![0.76780201f32, 0.80231093f32, 1.04554899f32, 0.33242197f32],
            vec![1.4940289f32 , 1.04554899f32, 1.80385488f32, 0.83237389f32],
            vec![0.75654844f32, 0.33242197f32, 0.83237389f32, 0.80474618f32]
        ]
    ).build().unwrap();
    let l = MatrixBuilder::new().from_mat(
        vec![
            vec![1.3922446173907894f64, 0.0f64                , 0.0f64                , 0.0f64               ], 
            vec![0.551484983640818f64 , 0.7058153132528383f64 , 0.0f64                , 0.0f64               ], 
            vec![1.0731080639375399f64, 0.6428679620108512f64 , 0.48889128612067473f64, 0.0f64               ], 
            vec![0.5434019674761598f64, 0.04639164479390491f64, 0.44881385275390756f64, 0.5530591230447315f64]
        ]
    ).build().unwrap();
    assert_eq!(l, a.cholesky().unwrap());
    let a = MatrixBuilder::new().from_mat(
        vec![
            vec![1.9383451f64 , 0.76780201f64, 1.4940289f64 , 0.75654844f64],
            vec![0.76780201f64, 0.80231093f64, 1.04554899f64, 0.33242197f64],
            vec![1.4940289f64 , 1.04554899f64, 1.80385488f64, 0.83237389f64],
            vec![0.75654844f64, 0.33242197f64, 0.83237389f64, 0.80474618f64]
        ]
    ).build().unwrap();
    let l = MatrixBuilder::new().from_mat(
        vec![
            vec![1.3922446264934910f64, 0.0f64                , 0.0f64                , 0.0f64               ],
            vec![0.5514849871848938f64, 0.705815300846955f64  , 0.0f64                , 0.0f64               ], 
            vec![1.0731080383214429f64, 0.6428679240784554f64 , 0.4888914504068998f64 , 0.0f64               ], 
            vec![0.5434019464707462f64, 0.04639167566223414f64, 0.44881368408590505f64, 0.5530593042145585f64],
        ]
    ).build().unwrap();
    assert_eq!(l, a.cholesky().unwrap());
}
