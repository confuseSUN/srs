use std::{fs::File, io::{BufReader, BufRead}};

use ark_bls12_381::{G1Affine, Fq, Fq2, G2Affine};
use num_bigint::BigUint;
use text_io::scan;
use process_path::{get_executable_path, get_dylib_path};

type G1 = ark_bls12_381::G1Projective;
type G2 = ark_bls12_381::G2Projective;

pub fn export_g1_from_public_setup_parameters(max_degree: usize) -> Vec<G1> {
    let mut path = get_executable_path().unwrap();
    path.push("public_setup_parameters/g1_coeffs.dat");
    println!("path = {:?}" , path);
    let file_in = File::open(path).unwrap();
    let mut reader = BufReader::new(file_in);

    let mut line = String::new();

    let mut g1 = Vec::<G1>::new();
    loop {
        let _ = reader.read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let x_str: String;
        let y_str: String;

        scan!(line.bytes() => "{} {}", x_str, y_str);

        assert!(x_str.starts_with("0x"));
        assert!(y_str.starts_with("0x"));

        let x = BigUint::parse_bytes(&x_str.as_bytes()[2..], 16).unwrap();
        let y = BigUint::parse_bytes(&y_str.as_bytes()[2..], 16).unwrap();

        let x_field_elem: Fq = x.into();
        let y_field_elem: Fq = y.into();

        g1.push(G1Affine::new(x_field_elem, y_field_elem, false).into());

        if g1.len() > max_degree {
            break;
        }

        line.clear();
    }

    g1
}

pub fn export_g2_from_public_setup_parameters() -> Vec<G2> {
    let mut path = get_dylib_path().unwrap();
    path.push("public_setup_parameters/g2_coeffs.dat");
    println!("path = {:?}" , path);
    let file_in = File::open(path).unwrap();
    let mut reader = BufReader::new(file_in);

    let mut line = String::new();

    let mut g2 = Vec::<G2>::new();

    for _ in 0..2 {
        let _ = reader.read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let x_c0_str: String;
        let x_c1_str: String;
        let y_c0_str: String;
        let y_c1_str: String;

        scan!(line.bytes() => "{} {} {} {}", x_c0_str, x_c1_str, y_c0_str, y_c1_str);

        assert!(x_c0_str.starts_with("0x"));
        assert!(x_c1_str.starts_with("0x"));
        assert!(y_c0_str.starts_with("0x"));
        assert!(y_c1_str.starts_with("0x"));

        let x_c0 = BigUint::parse_bytes(&x_c0_str.as_bytes()[2..], 16).unwrap();
        let x_c1 = BigUint::parse_bytes(&x_c1_str.as_bytes()[2..], 16).unwrap();

        let y_c0 = BigUint::parse_bytes(&y_c0_str.as_bytes()[2..], 16).unwrap();
        let y_c1 = BigUint::parse_bytes(&y_c1_str.as_bytes()[2..], 16).unwrap();

        let x_c0_field_elem: Fq = x_c0.into();
        let x_c1_field_elem: Fq = x_c1.into();

        let y_c0_field_elem: Fq = y_c0.into();
        let y_c1_field_elem: Fq = y_c1.into();

        let x_field_elem = Fq2::new(x_c0_field_elem, x_c1_field_elem);
        let y_field_elem = Fq2::new(y_c0_field_elem, y_c1_field_elem);

        let elem = G2Affine::new(x_field_elem, y_field_elem, false);
        assert!(elem.is_on_curve());
        g2.push(elem.into());

        line.clear();
    }

    g2
}