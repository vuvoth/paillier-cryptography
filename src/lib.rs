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



impl Paillier {
    pub fn keygen(bits: usize) -> Paillier {
        let p = BigNumber::safe_prime(bits);
        let q = BigNumber::safe_prime(bits);

        let n = p.clone() * q.clone();
    
        let pk = PaillierPK {
            g: n.clone() + BigNumber::one(),
            n: n.clone()
        };


        let phi = (p.clone() - BigNumber::one()) * (q.clone() - BigNumber::one());

        println!("{}", phi.to_string());
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

    pub fn encrypt(&self, m: &BigNumber) -> BigNumber {
        if !m.gcd(&self.pk.n).is_one() {
            panic!("message should coprime with n");
        }

        let n = self.pk.n.clone();
        let g = self.pk.g.clone();
        // gen random r 
        let mut r = BigNumber::random(&n);
        while !r.gcd(&self.pk.n).is_one() {
            r = BigNumber::random(&n);
        }

        let n_square = n.clone() * n.clone();
        
        let g_power = g.modpow(m, &n_square);
        let r_power = r.modpow(&n, &n_square);
        
        g_power.modmul(&r_power, &n_square)
    }

    pub fn decrypt(&self, c: &BigNumber) -> BigNumber {
        let phi = self.sk.phi.clone();
        let mi = self.sk.mi.clone();
        let n = self.sk.n.clone();
        let n_square = n.clone() * n.clone();
        let d = c.modpow(&phi, &n_square);
        let e = (d - BigNumber::one()) / n.clone();
        e.modmul(&mi, &n)
    }
}


#[cfg(test)]
mod tests {
    use crate::Paillier;
    use unknown_order::BigNumber;
    
    #[test]
    fn gen_key() {
        let paillier = Paillier::keygen(5);

        println!("{:?}", paillier);
        let m = BigNumber::from(2);
        let c = paillier.encrypt(&m);
        let d = paillier.decrypt(&c);
        println!("{:?}", m);
        println!("{:?}", c);
        println!("{:?}", d);
        assert!(m == d);
    }
}
