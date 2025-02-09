use unknown_order::BigNumber;


#[derive(Debug)]
pub struct PaillierPK {
    n: BigNumber,
    g: BigNumber
}

#[derive(Debug)]
pub struct PaillierSK {
    phi: BigNumber,
    mi: BigNumber,
    n: BigNumber
}

#[derive(Debug)]
pub struct Paillier {
    pk: PaillierPK,
    sk: PaillierSK
}

pub fn keygen(bits: usize) -> Paillier {
    let p = BigNumber::safe_prime(bits);
    let q = BigNumber::safe_prime(bits);

    let n = p.clone() * q.clone();
    
    let pk = PaillierPK {
        g: n.clone() + BigNumber::one(),
        n: n.clone()
    };


    let phi = (p.clone() - BigNumber::one()) * (q.clone() - BigNumber::one());
    let mi = phi.invert(&n).expect("should have invert");
    let sk = PaillierSK {
        phi,
        mi,
        n
    };

    Paillier {
        pk,
        sk
    }
}




#[cfg(test)]
mod tests {
    use crate::keygen;

    #[test]
    fn gen_key() {
        let pk = keygen(5);

        println!("{:?}", pk);
    }
}

