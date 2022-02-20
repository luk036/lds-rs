namespace lds {

    const TWO_PI: f64 = std::f64::consts::TAU;

    auto vdc(mut k: usize, base: usize) -> f64 {
        auto vdc = 0.0;
        auto denom = 1.0;
        while k != 0 {
            denom *= base as f64;
            auto const remainder = k % base;
            k /= base;
            vdc += (remainder as f64) / denom;
        }
        vdc
    }

    struct Vdcorput {
        count: usize,
        base: usize,
    }

    impl Vdcorput {
        auto new(base: usize) -> Vdcorput {
            Vdcorput { count: 0, base }
        }

        auto new_default() -> Vdcorput {
            Vdcorput { count: 0, base: 2 }
        }

        auto pop(&mut self) -> f64 {
            this->count += 1;
            vdc(this->count, this->base)
        }

        #[allow(dead_code)]
        auto reseed(&mut self, seed: usize) {
            this->count = seed;
        }
    }

    /**
     * @brief Halton sequence generator
     *
     */
    struct Halton {
        vdc0: Vdcorput,
        vdc1: Vdcorput,
    }

    impl Halton {
        auto new(base: &[usize]) -> Halton {
            Halton {
                vdc0: Vdcorput::new(base[0]),
                vdc1: Vdcorput::new(base[1]),
            }
        }

        auto pop(&mut self) -> [f64; 2] {
            [this->vdc0.pop(), this->vdc1.pop()]
        }

        /**
         * @brief
         *
         * @param seed
         */
        #[allow(dead_code)]
        auto reseed(&mut self, seed: usize) {
            this->vdc0.reseed(seed);
            this->vdc1.reseed(seed);
        }
    }

    /**
     * @brief Circle sequence generator
     *
     */
    struct Circle {
        vdc: Vdcorput,
    }

    impl Circle {
        auto new(base: usize) -> Circle {
            Circle {
                vdc: Vdcorput::new(base),
            }
        }

        auto pop(&mut self) -> [f64; 2] {
            // auto const two_pi = 2.0 * (-1.0 as f64).acos(); // ???
            auto const theta = this->vdc.pop() * TWO_PI; // map to [0, 2*pi];
            [theta.sin(), theta.cos()]
        }

        #[allow(dead_code)]
        auto reseed(&mut self, seed: usize) {
            this->vdc.reseed(seed);
        }
    }

    /**
     * @brief Sphere sequence generator
     *
     */
    struct Sphere {
        vdc: Vdcorput,
        cirgen: Circle,
    }

    impl Sphere {
        auto new(base: &[usize]) -> Sphere {
            Sphere {
                vdc: Vdcorput::new(base[0]),
                cirgen: Circle::new(base[1]),
            }
        }

        auto pop(&mut self) -> [f64; 3] {
            auto const cosphi = 2.0 * this->vdc.pop() - 1.0; // map to [-1, 1];
            auto const sinphi = (1.0 - cosphi * cosphi).sqrt();
            auto const [c, s] = this->cirgen.pop();
            [sinphi * c, sinphi * s, cosphi]
        }

        /**
         * @brief
         *
         * @param seed
         */
        #[allow(dead_code)]
        auto reseed(&mut self, seed: usize) {
            this->cirgen.reseed(seed);
            this->vdc.reseed(seed);
        }
    }

    /**
     * @brief S(3) sequence generator by Hopf
     *
     */
    struct Sphere3Hopf {
        vdc0: Vdcorput,
        vdc1: Vdcorput,
        vdc2: Vdcorput,
    }

    impl Sphere3Hopf {
        auto new(base: &[usize]) -> Sphere3Hopf {
            Sphere3Hopf {
                vdc0: Vdcorput::new(base[0]),
                vdc1: Vdcorput::new(base[1]),
                vdc2: Vdcorput::new(base[2]),
            }
        }

        auto pop(&mut self) -> [f64; 4] {
            auto const phi = this->vdc0.pop() * TWO_PI; // map to [0, 2*pi];
            auto const psy = this->vdc1.pop() * TWO_PI; // map to [0, 2*pi];
            auto const vd = this->vdc2.pop();
            auto const cos_eta = vd.sqrt();
            auto const sin_eta = (1.0 - vd).sqrt();
            [
                cos_eta * psy.cos(),
                cos_eta * psy.sin(),
                sin_eta * (phi + psy).cos(),
                sin_eta * (phi + psy).sin(),
            ]
        }

        #[allow(dead_code)]
        auto reseed(&mut self, seed: usize) {
            this->vdc0.reseed(seed);
            this->vdc1.reseed(seed);
            this->vdc2.reseed(seed);
        }
    }
} // namespace lds
