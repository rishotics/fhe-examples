use tfhe::{prelude::*, ClientKey};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8};



//fn to encrypt a matrix with server keys
fn encrypt_matrix(matrix: Vec<Vec<u32>>, client_key: &ClientKey) -> Vec<Vec<FheUint32>> {
    let mut encrypted_matrix = Vec::new();
    for row in matrix {
        let mut encrypted_row = Vec::new();
        for element in row {
            encrypted_row.push(FheUint32::try_encrypt(element, client_key).unwrap());
        }
        encrypted_matrix.push(encrypted_row);
    }
    encrypted_matrix
}

//multiply a vector with encrypted matrix
fn multiply_vector_with_matrix(vector: Vec<u32>, encrypted_matrix: Vec<Vec<FheUint32>>) -> Vec<FheUint32> {
    let mut result = Vec::new();
    for row in encrypted_matrix {
        let mut sum = FheUint32::sum(row);
        for (i, element) in row.iter().enumerate() {
            sum = &sum + &(&element * vector[i]);
        }
        result.push(sum);
    }
    result
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let clear_a = 1344u32;
    let clear_b = 5u32;
    let clear_c = 7u8;

    let encrypted_matrix = encrypt_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7,8,9]], &client_key);

    set_server_key(server_keys);

    let vector = vec![1, 2, 3];

    let result = multiply_vector_with_matrix(vector, encrypted_matrix);

    for row in result {
        // let clear_res: u8 = encrypted_res.decrypt(&client_key);
        println!("{:?}", row.decrypt(&client_key));
    }
    // Encrypting the input data using the (private) client_key
    // FheUint32: Encrypted equivalent to u32
    // let mut encrypted_a = FheUint32::try_encrypt(clear_a, &client_key)?;
    // let encrypted_b = FheUint32::try_encrypt(clear_b, &client_key)?;

    // // FheUint8: Encrypted equivalent to u8
    // let encrypted_c = FheUint8::try_encrypt(clear_c, &client_key)?;

    // // On the server side:
    // set_server_key(server_keys);

    

    // // Clear equivalent computations: 1344 * 5 = 6720
    // let encrypted_res_mul = &encrypted_a * &encrypted_b;

    // // Clear equivalent computations: 1344 >> 5 = 42
    // encrypted_a = &encrypted_res_mul >> &encrypted_b;

    // // Clear equivalent computations: let casted_a = a as u8;
    // let casted_a: FheUint8 = encrypted_a.cast_into();

    // // Clear equivalent computations: min(42, 7) = 7
    // let encrypted_res_min = casted_a.min(&encrypted_c);

    // // Operation between clear and encrypted data:
    // // Clear equivalent computations: 7 & 1 = 1
    // let encrypted_res = encrypted_res_min & 1_u8;

    // // println!("Encrypted result: {:?}", encrypted_res.);

    // // Decrypting on the client side:
    // let clear_res: u8 = encrypted_res.decrypt(&client_key);
    // assert_eq!(clear_res, 1_u8);

    Ok(())
}